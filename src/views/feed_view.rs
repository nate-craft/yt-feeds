use colored::Colorize;
use itertools::Itertools;

use crate::{
    clear_screen,
    page::Page,
    utilities::{time_formatted_short, time_since_formatted},
    view::{Message, ViewPage},
    yt::{ChannelIndex, Channels, Video, VideoIndex},
};

use super::View;

pub fn show_channel(channel_index: ChannelIndex, channels: &Channels) -> Message {
    let channel = channels.channel(channel_index).unwrap();

    let videos: Vec<(usize, &Video)> = channel.videos.iter().enumerate().collect();

    clear_screen();

    let mut page = Page::new(10, videos.len(), 3);
    let mut view = View::new(
        format!("{}'s Feed", &channel.name).as_str(),
        "(p)revious, (n)ext, (r)efresh, (c)hannels, (u)nsubscribe, (q)uit",
        "🢡",
    );

    loop {
        view.clear_content();

        page.current_page(&videos)
            .iter()
            .enumerate()
            .map(|(i, video)| (i + page.current_index, video))
            .for_each(|(i, (_, video))| {
                if video.watched {
                    view.add_line(format!(
                        "{}. {}\n   {} • {}\n",
                        i.to_string().green(),
                        video.title.bright_yellow().underline(),
                        time_since_formatted(video.upload),
                        time_formatted_short(video.progress_seconds)
                    ));
                } else {
                    view.add_line(format!(
                        "{}. {}\n   {} • {}\n",
                        i.to_string().green(),
                        video.title.yellow(),
                        time_since_formatted(video.upload),
                        time_formatted_short(video.progress_seconds)
                    ));
                }
            });

        match view.show().to_lowercase().as_str() {
            "q" => return Message::Quit,
            "c" => return Message::Home,
            "u" => return Message::Unsubscribe(channel_index),
            "r" => return Message::Refresh(ViewPage::FeedChannel(channel_index)),
            "n" => {
                page.next_page();
                view.clear_error();
            }
            "p" => {
                page.prev_page();
                view.clear_error();
            }
            input => {
                if let Ok(index) = &input.parse::<usize>() {
                    if page.item_is_at_index(*index) {
                        return Message::Play(VideoIndex {
                            channel_index: channel_index.0,
                            video_index: *index,
                        });
                    }
                }
                view.set_error(format!("{} is not a valid option!", input));
            }
        }
    }
}

pub fn show_mixed(channels: &Channels) -> Message {
    let videos: Vec<(usize, usize, &String, &Video)> = channels
        .iter()
        .enumerate()
        .flat_map(|(i, channel)| -> Vec<(usize, usize, &String, &Video)> {
            channel
                .videos
                .iter()
                .enumerate()
                .map(|(j, video)| (i, j, &channel.name, video))
                .collect()
        })
        .sorted_by(|a, b| b.3.upload.cmp(&a.3.upload))
        .collect();

    let mut page = Page::new(10, videos.len(), 3);

    let mut view = View::new(
        "Subscription Feed",
        "(p)revious, (n)ext, (r)efresh, (c)hannels, (q)uit",
        "🢡",
    );

    loop {
        view.clear_content();
        page.current_page(&videos)
            .iter()
            .enumerate()
            .map(|(i, video)| (i + page.current_index, video))
            .for_each(|(i, (_, _, channel, video))| {
                if video.watched {
                    view.add_line(format!(
                        "{}. {}\n   {} • {} • {}\n",
                        i.to_string().green(),
                        video.title.bright_yellow().underline(),
                        channel,
                        time_since_formatted(video.upload),
                        time_formatted_short(video.progress_seconds)
                    ));
                } else {
                    view.add_line(format!(
                        "{}. {}\n   {} • {} • {}\n",
                        i.to_string().green(),
                        video.title.yellow(),
                        channel,
                        time_since_formatted(video.upload),
                        time_formatted_short(video.progress_seconds)
                    ));
                }
            });

        match view.show().to_lowercase().as_str() {
            "q" => return Message::Quit,
            "c" => return Message::Home,
            "r" => return Message::Refresh(ViewPage::MixedFeed),
            "n" => {
                page.next_page();
                view.clear_error();
            }
            "p" => {
                page.prev_page();
                view.clear_error();
            }
            input => {
                if let Ok(index) = &input.parse::<usize>() {
                    let item = page.item_at_index(&videos, *index);
                    if let Some((channel_index, video_index, _, _)) = item {
                        return Message::Play(VideoIndex {
                            channel_index: *channel_index,
                            video_index: *video_index,
                        });
                    }
                }

                view.set_error(format!("{} is not a valid option!", input));
            }
        }
    }
}
