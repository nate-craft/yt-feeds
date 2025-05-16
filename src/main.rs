use core::panic;
use std::ops::Deref;
use std::path::PathBuf;
use std::rc::Rc;
use std::{io, process, sync::mpsc};

use config::Config;
use crossterm::execute;
use crossterm::{
    cursor,
    terminal::{self, ClearType},
};
use updates::{check_updates, fetch_updates};
use view::{Message, ViewPage};
use views::{feed_view, home_view, player_view, search_view};
use yt::{Channel, Channels};

mod cache;
mod config;
mod loading;
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

fn main() {
    let mut history = cache::fetch_watch_history().unwrap_or(Vec::new());

    let config = match Config::load_or_default() {
        Ok(loaded) => loaded,
        Err(err) => {
            panic!(
                "Could not retrieve local config directory. \nError: {:?}",
                err
            );
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

    state.channels.add_history(&history);

    let (tx, rx) = mpsc::channel::<Channel>();

    // Auto update and auto cache on startup
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

    loop {
        // check for updates in the background
        check_updates(&rx, &mut state.channels, false);

        let message: Message = match state.view {
            ViewPage::Home => home_view::show(&state.channels),
            ViewPage::FeedChannel(channel_index) => {
                feed_view::show_channel(channel_index, &state.channels)
            }

            ViewPage::MixedFeed => feed_view::show_mixed(&state.channels),
            ViewPage::Search => search_view::show(&state.channels),
            ViewPage::Play(video_index, ref last_view) => {
                let next = player_view::show(&state.channels, video_index, last_view.as_ref());
                //TODO: add optimization to only add history for specific video / on finish playing
                history = cache::fetch_watch_history().unwrap_or(Vec::new());
                state.channels.add_history(&history);
                next
            }
            ViewPage::Refreshing(ref last_view) => {
                fetch_updates(
                    tx.clone(),
                    state
                        .channels
                        .0
                        .iter()
                        .map(|channel| channel.into())
                        .collect(),
                    config.video_count,
                );
                check_updates(&rx, &mut state.channels, true);
                match last_view.deref() {
                    ViewPage::Home => Message::Home,
                    ViewPage::FeedChannel(channel_index) => Message::ChannelFeed(*channel_index),
                    ViewPage::MixedFeed => Message::MixedFeed,
                    _ => panic!(),
                }
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
                    eprintln!(
                        "Could not retrieve local data directory. Caching cannot be enabled!\nError: {:?}",
                        err
                    );
                }
            }
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
        Message::Quit => exit(),
    }
}

fn try_cache_channels(channels: &Channels) {
    if let Err(err) = cache::cache_channels(channels) {
        eprintln!(
            "Could not retrieve local data directory. Caching cannot be enabled!\nError: {:?}",
            err
        );
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

fn exit() -> ! {
    clear_screen();
    process::exit(0);
}
