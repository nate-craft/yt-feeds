use colored::Colorize;
use itertools::Itertools;

use crate::{
    clear_screen,
    page::Page,
    utilities::{time_formatted_short, time_since_formatted},
    view::{Message, ViewPage},
    yt::{ChannelIndex, Channels, Video, VideoIndex},
};

use super::{View, ViewInput};

pub fn show_channel(channel_index: ChannelIndex, channels: &Channels) -> Message {
    let channel = channels.channel(channel_index).unwrap();

    let videos: Vec<(usize, &Video)> = channel.videos.iter().enumerate().collect();

    clear_screen();

    let mut page = Page::new(videos.len(), 3);
    let mut view = View::new(
        format!("{}'s Feed", &channel.name),
        "(p)revious, (n)ext, (r)efresh, (u)nsubscribe, (b)ack, (q)uit".to_owned(),
        "▶".to_owned(),
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

        match view.show() {
            ViewInput::Char(char) => match char {
                'q' => return Message::Quit,
                'b' => return Message::Home,
                'u' => return Message::Unsubscribe(channel_index),
                'r' => return Message::Refresh(ViewPage::FeedChannel(channel_index)),
                'n' => {
                    page.next_page();
                    view.clear_error();
                }
                'p' => {
                    page.prev_page();
                    view.clear_error();
                }
                input => {
                    view.set_error(&format!("{} is not a valid option!", input));
                }
            },
            ViewInput::Num(num) => {
                if page.item_is_at_index(num) {
                    return Message::Play(VideoIndex {
                        channel_index: channel_index.0,
                        video_index: num,
                    });
                }
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

    let mut page = Page::new(videos.len(), 3);

    let mut view = View::new(
        "Subscription Feed".to_owned(),
        "(p)revious, (n)ext, (r)efresh, (b)ack, (q)uit".to_owned(),
        "▶".to_owned(),
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

        match view.show() {
            ViewInput::Char(char) => match char {
                'q' => return Message::Quit,
                'b' => return Message::Home,
                'r' => return Message::Refresh(ViewPage::MixedFeed),
                'n' => {
                    page.next_page();
                    view.clear_error();
                }
                'p' => {
                    page.prev_page();
                    view.clear_error();
                }
                input => {
                    view.set_error(&format!("{} is not a valid option!", input));
                }
            },
            ViewInput::Num(num) => {
                let item = page.item_at_index(&videos, num);
                if let Some((channel_index, video_index, _, _)) = item {
                    return Message::Play(VideoIndex {
                        channel_index: *channel_index,
                        video_index: *video_index,
                    });
                }
            }
        }
    }
}
