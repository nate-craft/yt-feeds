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
use views::{feed_view, home_view, information_view, player_view, search_view};
use yt::{Channel, Channels};

mod cache;
mod config;
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

    let channels_cached = cache::fetch_cached_channels();

    let mut state = if let Some(channels_cached) = channels_cached {
        AppState {
            channels: Channels::new(&channels_cached),
            view: ViewPage::Home,
            root_dir: cache::data_directory().ok(),
        }
    } else {
        AppState {
            channels: Channels::default(),
            view: ViewPage::Search,
            root_dir: cache::data_directory().ok(),
        }
    };

    let (tx, rx) = mpsc::channel::<Channel>();

    // Auto update on startup
    if config.refresh_on_start {
        updates::fetch_updates(
            tx.clone(),
            state
                .channels
                .iter()
                .map(|channel| channel.into())
                .collect(),
            config.video_count,
        );
        try_cache_channels(&state.channels);
    }

    loop {
        // check for auto updates in background of each loop
        if config.refresh_on_start {
            check_updates(&rx, &mut state.channels, Blocking::NoWait);
        }

        let message: Message = match state.view {
            ViewPage::Home => home_view::show(&state.channels),
            ViewPage::FeedChannel(channel_index) => {
                feed_view::show_channel(channel_index, &state.channels)
            }
            ViewPage::MixedFeed => feed_view::show_mixed(&state.channels),
            ViewPage::Search => search_view::show(&state.channels),
            ViewPage::Play(video_index, ref last_view) => {
                let next = player_view::show(&state.channels, video_index, &last_view, &config);

                let channel = state.channels.channel_mut(video_index.into()).unwrap();
                let video = channel.video_mut(video_index).unwrap();
                let history_fetched = cache::fetch_history_one(&video.id);
                match history_fetched {
                    Ok(history_fetched) => {
                        video.progress_seconds = Some(history_fetched.progress_seconds)
                    }
                    Err(e) => log::err(format!("Could not fetch watch history.\nError: {:?}", e)),
                }
                next
            }
            ViewPage::Refreshing(ref last_view) => {
                if let ViewPage::FeedChannel(channel_index) = **last_view {
                    let channel = state.channels.channel(channel_index).unwrap();
                    fetch_updates(tx.clone(), vec![channel.into()], config.video_count);
                    check_updates(&rx, &mut state.channels, Blocking::WaitForN(1));
                } else {
                    fetch_updates(
                        tx.clone(),
                        state
                            .channels
                            .iter()
                            .map(|channel| channel.into())
                            .collect(),
                        config.video_count,
                    );

                    let number_updates = state.channels.len();
                    check_updates(&rx, &mut state.channels, Blocking::WaitForN(number_updates));
                }

                match **last_view {
                    ViewPage::Home => Message::Home,
                    ViewPage::FeedChannel(channel_index) => Message::ChannelFeed(channel_index),
                    ViewPage::MixedFeed => Message::MixedFeed,
                    _ => panic!(),
                }
            }
            ViewPage::Information(video_index, ref last_view) => {
                information_view::show(&state.channels, video_index, last_view.clone())
            }
        };

        handle_message(message, &mut state);
    }
}

fn handle_message(message: Message, state: &mut AppState) {
    match message {
        Message::MixedFeed => state.view = ViewPage::MixedFeed,
        Message::Home => state.view = ViewPage::Home,
        Message::ChannelFeed(channel_index) => state.view = ViewPage::FeedChannel(channel_index),
        Message::Search => state.view = ViewPage::Search,
        Message::Play(video_index) => {
            let channel = state.channels.channel_mut(video_index.into()).unwrap();
            channel.video_mut(video_index).unwrap().watched();
            state.view = ViewPage::Play(video_index, Rc::new(state.view.clone()));

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
            state.view = ViewPage::Refreshing(Rc::new(last_view));
            try_cache_channels(&state.channels);
        }
        Message::Quit => {
            clear_screen();
            process::exit(0);
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
