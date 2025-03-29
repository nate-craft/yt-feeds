use std::{
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

pub fn fetch_updates(tx: Sender<Channel>, channels: Vec<ChannelInfo>, video_count: u32) {
    channels.into_iter().for_each(|channel| {
        let tx = tx.clone();
        thread::spawn(move || {
            let feed = yt::feed_channel(&channel.id, video_count);
            match feed {
                Ok(feed) => {
                    tx.send(Channel::new(channel.name, channel.id, feed))
                        .unwrap();
                }
                Err(err) => match err {
                    Error::ChannelParsing => {
                        eprintln!(
                            "{}: '{}'",
                            "Could not find videos for channel", channel.name
                        );
                    }
                    Error::CommandFailed => {
                        eprintln!(
                            "{}: '{}' with command 'yt-dlp'",
                            "Could not load in feed for channel", channel.id,
                        );
                    }
                    _ => {}
                },
            }
        });
    });

    drop(tx);
}

pub fn check_updates(rx: &Receiver<Channel>, channels: &mut Channels, blocking: bool) {
    if blocking {
        let mut updated = 0;
        let mut step = 0;
        let steps = ["⢿", "⣻", "⣽", "⣾", "⣷", "⣯", "⣟", "⡿"];

        // block till all channels are refreshed
        while updated < channels.len() {
            if let Ok(fetched) = rx.try_recv() {
                // more updates than there are channels (old updates)
                if updated >= channels.len() {
                    thread::sleep(Duration::from_secs(1));
                    return;
                }

                if let Some(existing) = channels.channel_by_id_mut(&fetched.id) {
                    for new_video in &fetched.videos {
                        if !existing
                            .videos
                            .iter()
                            .any(|existing_video| existing_video.url == new_video.url)
                        {
                            existing.videos.push(new_video.clone());
                        }
                    }

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
    } else {
        while let Ok(fetched) = rx.try_recv() {
            if let Some(existing) = channels.channel_by_id_mut(&fetched.id) {
                for new_video in &fetched.videos {
                    if !existing
                        .videos
                        .iter()
                        .any(|existing_video| existing_video.url == new_video.url)
                    {
                        existing.videos.push(new_video.clone());
                    }
                }
            }
        }
    }
}
