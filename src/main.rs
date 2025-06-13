use core::panic;
use std::path::PathBuf;
use std::process::{self, Command};
use std::rc::Rc;
use std::thread;
use std::{io, sync::mpsc};

use config::Config;
use crossterm::execute;
use crossterm::style::Stylize;
use crossterm::{
    cursor,
    terminal::{self, ClearType},
};
use updates::{check_updates, fetch_updates, Blocking};
use view::{Message, ViewPage};
use views::{feed_view, home_view, information_view, player_view, search_channel_view};
use yt::{Channel, Channels};

use crate::loading::run_while_loading;
use crate::view::{LastSearch, PlayType};
use crate::views::{search_video_view, watch_later_view};
use crate::yt::{fetch_more_videos, VideoWatchLater};

mod cache;
mod config;
mod finder;
mod loading;
mod log;
mod page;
mod search;
mod updates;
mod utilities;
mod view;
mod views;
mod yt;

pub struct AppState {
    channels: Channels,
    view: ViewPage,
    root_dir: Option<PathBuf>,
    last_search: Option<LastSearch>,
    watch_later: Vec<VideoWatchLater>,
    tx: mpsc::Sender<Channel>,
    rx: mpsc::Receiver<Channel>,
}

impl AppState {
    pub fn new() -> Self {
        let channels_cached = cache::fetch_cached_channels();
        let (tx, rx) = mpsc::channel::<Channel>();

        if let Some(channels_cached) = channels_cached {
            AppState {
                channels: Channels::new(&channels_cached),
                view: ViewPage::Home,
                root_dir: cache::data_directory().ok(),
                last_search: None,
                watch_later: cache::fetch_watch_later_videos(),
                tx,
                rx,
            }
        } else {
            AppState {
                channels: Channels::default(),
                view: ViewPage::SearchChannels,
                root_dir: cache::data_directory().ok(),
                watch_later: cache::fetch_watch_later_videos(),
                last_search: None,
                tx,
                rx,
            }
        }
    }
}

fn program_installed(command: &str) -> bool {
    Command::new(command).arg("--version").output().is_ok()
}

fn main() {
    thread::spawn(|| {
        if !program_installed("mpv") {
            log::err_and_exit("mpv must be installed and locatable on your PATH.\nFor help, visit https://github.com/higgsbi/yt-feeds".red());
        }

        if !program_installed("yt-dlp") {
            log::err_and_exit("yt-dlp must be installed and locatable on your PATH.\nFor help, visit https://github.com/higgsbi/yt-feeds".red());
        }
    });

    let config = match Config::load_or_default() {
        Ok(loaded) => loaded,
        Err(err) => {
            log::err_and_exit(format!(
                "Could not retrieve local config directory. \nError: {:?}",
                err
            ));
        }
    };

    let mut state = AppState::new();

    // Auto update on startup
    if config.refresh_on_start {
        updates::fetch_updates(
            state.tx.clone(),
            state
                .channels
                .iter()
                .map(|channel| channel.into())
                .collect(),
            config.videos_per_channel,
        );
        try_cache_channels(&state.channels);
    }

    loop {
        // check for auto updates in background of each loop
        if config.refresh_on_start {
            check_updates(&state.rx, &mut state.channels, Blocking::NoWait);
        }

        let message: Message = match state.view {
            ViewPage::Home => home_view::show(&state.channels),
            ViewPage::SearchChannels => search_channel_view::show(&state.channels, &config),
            ViewPage::SearchVideos => search_video_view::show(&config, state.last_search.as_ref()),
            ViewPage::WatchLater => watch_later_view::show(&state.watch_later),
            ViewPage::Refreshing(ref last_view) => last_view.as_ref().clone().into(),
            ViewPage::MixedFeed(last_index) => feed_view::show_mixed(&state.channels, last_index),
            ViewPage::ChannelFeed(channel_index, last_index) => {
                feed_view::show_channel(channel_index, &state.channels, last_index)
            }
            ViewPage::Play(ref play_type, ref last_view) => player_view::show(
                &state.channels,
                &state.watch_later,
                play_type,
                &last_view,
                &config,
            ),
            ViewPage::Information(video_index, ref last_view) => {
                information_view::show(&state.channels, video_index, last_view.clone())
            }
        };

        handle_message(message, &mut state, &config);
    }
}

fn handle_message(message: Message, state: &mut AppState, config: &Config) {
    match message {
        Message::Home => {
            state.last_search = None;
            state.view = ViewPage::Home
        }
        Message::MixedFeed(last_index) => state.view = ViewPage::MixedFeed(last_index),
        Message::ChannelFeed(channel_index, last_index) => {
            state.view = ViewPage::ChannelFeed(channel_index, last_index)
        }
        Message::WatchLater => state.view = ViewPage::WatchLater,
        Message::SearchChannels => state.view = ViewPage::SearchChannels,
        Message::SearchVideos => state.view = ViewPage::SearchVideos,
        Message::SearchVideosClean => {
            state.view = ViewPage::SearchVideos;
            state.last_search = None;
        }
        Message::WatchLaterRemove(index) => {
            state.view = ViewPage::WatchLater;
            state.watch_later.remove(index);

            if let Some(root) = &state.root_dir {
                if let Err(err) = cache::cache_watch_later(root, &state.watch_later) {
                    log::err(format!(
                        "Could not cache watch_history. Progress will not be saved!\nError: {:?}",
                        err
                    ));
                }
            }
        }
        Message::WatchLaterAdd(video_info, last_view) => {
            state.view = (*last_view).clone();
            state.watch_later.push(video_info);

            if let Some(root) = &state.root_dir {
                if let Err(err) = cache::cache_watch_later(&root, &state.watch_later) {
                    log::err(format!(
                        "Could not cache watch_history. Progress will not be saved!\nError: {:?}",
                        err
                    ));
                }
            }
        }
        Message::Play(play_type) => {
            if let PlayType::New(_, cached_search) = &play_type {
                state.last_search = cached_search.clone();
            }
            state.view = ViewPage::Play(play_type, Rc::new(state.view.clone()));
        }
        Message::Played(view_page, video_index) => {
            state.view = view_page.as_ref().to_owned();

            // do not cache single searched video playing
            let Some(video_index) = video_index else {
                return;
            };

            // mark watched
            let channel = state.channels.channel_mut(video_index.into()).unwrap();
            let video = channel.video_mut(video_index).unwrap();
            video.watched();

            // mark progress
            let history_fetched = cache::fetch_history_one(&video.id);
            match history_fetched {
                Ok(history_fetched) => {
                    video.progress_seconds = Some(history_fetched.progress_seconds)
                }
                Err(e) => log::err(format!("Could not fetch watch history.\nError: {:?}", e)),
            }

            // cache singular channel
            if let Some(root) = &state.root_dir {
                if let Err(err) = cache::cache_videos(root, &channel.id, &channel.videos) {
                    log::err(format!(
                            "Could not retrieve local data directory. Caching cannot be enabled!\nError: {:?}",
                            err
                    ));
                }
            }
        }
        Message::Information(video_index, view_page) => {
            state.view = ViewPage::Information(video_index, view_page);
        }
        Message::MoreInformation(video_index, view_page, new_description) => {
            let channel = state.channels.channel_mut(video_index.into()).unwrap();
            let video = channel.video_mut(video_index).unwrap();
            video.description = new_description;
            state.view = ViewPage::Information(video_index, view_page);
        }
        Message::Subscribe(channel) => {
            state.channels.push(channel);
            state.view = ViewPage::Home;
            try_cache_channels(&state.channels);
        }
        Message::Unsubscribe(channel_index) => {
            state.channels.remove(*channel_index);
            state.view = ViewPage::Home;
            try_cache_channels(&state.channels);
        }
        Message::Refresh(last_view) => {
            state.view = ViewPage::Refreshing(Rc::new(last_view.clone()));
            if let ViewPage::ChannelFeed(channel_index, _) = last_view {
                let channel = state.channels.channel(channel_index).unwrap();
                fetch_updates(
                    state.tx.clone(),
                    vec![channel.into()],
                    config.videos_per_channel,
                );
                check_updates(&state.rx, &mut state.channels, Blocking::WaitForN(1));
            } else {
                fetch_updates(
                    state.tx.clone(),
                    state
                        .channels
                        .iter()
                        .map(|channel| channel.into())
                        .collect(),
                    config.videos_per_channel,
                );

                let number_updates = state.channels.len();
                check_updates(
                    &state.rx,
                    &mut state.channels,
                    Blocking::WaitForN(number_updates),
                );
            }
            try_cache_channels(&state.channels);
        }
        Message::Quit => {
            clear_screen();
            process::exit(0);
        }
        Message::MoreVideos(channel_index, view_page, video_count, last_viewed_index) => {
            let channel = state.channels.channel_mut(channel_index).unwrap();
            let name = channel.name.clone();

            let success = run_while_loading(
                || fetch_more_videos(config, video_count, channel),
                move || {
                    println!(
                        "{}{}\n",
                        name.clone().cyan().bold(),
                        "'s - Feed".cyan().bold()
                    );
                    print!(
                        "{} {}",
                        "Fetching more videos for ".green(),
                        name.clone().yellow()
                    );
                },
            );

            if success {
                try_cache_channels(&state.channels);
            }

            state.view = match view_page {
                ViewPage::ChannelFeed(channel_index, _) => {
                    ViewPage::ChannelFeed(channel_index, Some(last_viewed_index))
                }
                ViewPage::MixedFeed(_) => ViewPage::MixedFeed(Some(last_viewed_index)),
                _ => panic!(),
            }
        }
    }
}

fn try_cache_channels(channels: &Channels) {
    if let Err(err) = cache::cache_channels(channels) {
        log::err(format!(
            "Could not retrieve local data directory. Caching cannot be enabled!\nError: {:?}",
            err
        ));
    }
}

fn clear_screen() {
    execute!(
        io::stdout(),
        terminal::Clear(ClearType::Purge),
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )
    .expect("Could not clear screen")
}
