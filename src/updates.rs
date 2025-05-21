use std::{
    cmp::min,
    sync::mpsc::{Receiver, Sender},
    thread,
    time::Duration,
};

use colored::Colorize;

use crate::{
    clear_screen,
    view::Error,
    yt::{ChannelInfo, Channels},
};
use crate::{yt, Channel};

#[derive(PartialEq, Eq)]
pub enum Blocking {
    WaitForN(usize),
    NoWait,
}

pub fn fetch_updates(tx: Sender<Channel>, channels: Vec<ChannelInfo>, video_count: u32) {
    channels.into_iter().for_each(|channel| {
        let tx = tx.clone();
        thread::spawn(move || {
            let feed = yt::fetch_channel_feed(&channel.id, video_count);
            match feed {
                Ok(feed) => {
                    tx.send(Channel::new(channel.name, channel.id, feed))
                        .unwrap();
                }
                Err(err) => match err {
                    Error::HistoryParsing => {
                        eprintln!(
                            "Could not find videos for channel: '{}'",
                            channel.name
                        );
                    }
                    Error::CommandFailed(e) => {
                        eprintln!(
                            "Could not load in feed for channel: '{}' with command 'yt-dlp'.\nError: {}",
                            channel.id, e
                        );
                    }
                    _ => {}
                },
            }
        });
    });

    drop(tx);
}

pub fn check_updates(rx: &Receiver<Channel>, channels: &mut Channels, blocking: Blocking) {
    match blocking {
        Blocking::WaitForN(number) => {
            let mut updated = 0;
            let mut step = 0;
            let steps = ["⢿", "⣻", "⣽", "⣾", "⣷", "⣯", "⣟", "⡿"];

            // block till specified number of channels are refreshed
            // TODO: ensure optional auto refresh on startup is done - if not, updated channel may be from old update
            while updated < min(number, channels.len()) {
                if let Ok(mut fetched) = rx.try_recv() {
                    // queue of updates more than channels exist (old stacked updates) -> give fake refresh time and return
                    if updated >= channels.len() {
                        thread::sleep(Duration::from_secs(1));
                        return;
                    }

                    if let Some(existing) = channels.channel_by_id_mut(&fetched.id) {
                        for new_video in &fetched.videos {
                            if !existing
                                .videos
                                .iter()
                                .any(|existing_video| existing_video.id == new_video.id)
                            {
                                existing.videos.push(new_video.clone());
                            }
                        }
                        fetched.videos.sort_by(|a, b| b.upload.cmp(&a.upload));
                        updated += 1;
                    }
                }

                clear_screen();
                println!("{}", "\nRefreshing Channels\n".cyan().bold());
                println!(
                    "{} {}  {}\n",
                    "Channels Updated:".green(),
                    updated.to_string().yellow(),
                    steps[step]
                );

                step = step + 1;
                if step > steps.len() - 1 {
                    step = 0;
                }
                thread::sleep(Duration::from_millis(450));
            }
        }
        Blocking::NoWait => {
            while let Ok(mut fetched) = rx.try_recv() {
                if let Some(existing) = channels.channel_by_id_mut(&fetched.id) {
                    for new_video in &fetched.videos {
                        if !existing
                            .videos
                            .iter()
                            .any(|existing_video| existing_video.id == new_video.id)
                        {
                            existing.videos.push(new_video.clone());
                        }
                    }
                    fetched.videos.sort_by(|a, b| b.upload.cmp(&a.upload));
                }
            }
        }
    }
}
