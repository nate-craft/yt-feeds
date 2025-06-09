use std::cmp::Ordering;

use crossterm::style::{Color, Stylize};
use itertools::Itertools;

use crate::{
    clear_screen,
    finder::Finder,
    page::Page,
    utilities::{self, time_formatted_short, time_since_formatted},
    view::{Message, PlayType, ViewPage},
    yt::{ChannelIndex, Channels, Video, VideoIndex},
};

use super::{View, ViewInput};

#[derive(Clone)]
enum VideoEntry<'a> {
    Mixed(usize, usize, &'a String, &'a Video),
    Channel(usize, &'a Video),
}

impl<'a> VideoEntry<'a> {
    fn get_video(&'a self) -> &'a Video {
        match self {
            VideoEntry::Mixed(_, _, _, video) => *video,
            VideoEntry::Channel(_, video) => *video,
        }
    }

    fn get_channel(&'a self) -> Option<&'a str> {
        match self {
            VideoEntry::Mixed(_, _, channel, _) => Some(channel),
            VideoEntry::Channel(_, _) => None,
        }
    }

    fn get_title_formatted(&'a self, query: Option<&str>) -> String {
        let video = self.get_video();
        if video.watched {
            utilities::highlight_query(&video.title, query, Some(Color::Yellow))
        } else {
            utilities::highlight_query(&video.title, query, Some(Color::DarkYellow))
        }
    }

    fn cmp(&'a self, other: &'a VideoEntry) -> Ordering {
        other.get_video().upload.cmp(&self.get_video().upload)
    }
}

pub fn show_channel(
    channel_index: ChannelIndex,
    channels: &Channels,
    last_index: Option<usize>,
) -> Message {
    let videos = channels
        .channel(channel_index)
        .unwrap()
        .videos
        .iter()
        .enumerate()
        .map(|(i, video)| VideoEntry::Channel(i, video))
        .collect();

    show_feed(&videos, channels, last_index, Some(channel_index))
}

pub fn show_mixed(channels: &Channels, last_index: Option<usize>) -> Message {
    let videos: Vec<VideoEntry> = channels
        .iter()
        .enumerate()
        .flat_map(|(i, channel)| -> Vec<VideoEntry> {
            channel
                .videos
                .iter()
                .enumerate()
                .map(|(j, video)| VideoEntry::Mixed(i, j, &channel.name, video))
                .collect()
        })
        .sorted_by(|a, b| a.cmp(b))
        .collect();

    show_feed(&videos, channels, last_index, None)
}

fn show_feed(
    videos: &Vec<VideoEntry>,
    channels: &Channels,
    last_index: Option<usize>,
    channel_index: Option<ChannelIndex>,
) -> Message {
    let mut page_normal = Page::new(videos.len(), 3);
    let mut finder = Finder::new(videos.len(), 3);

    page_normal.current_index = last_index.unwrap_or(page_normal.current_index);
    let channel = channel_index.map(|index| channels.channel(index).unwrap());

    let mut view = if let Some(channel) = channel {
        View::new(
            format!("{}'s Feed", &channel.name),
            "(p)revious, (n)ext, (m)ore, (f)ind, (r)efresh, (u)nsubscribe, (b)ack, (q)uit"
                .to_owned(),
            "▶".to_owned(),
        )
    } else {
        View::new(
            "Subscription Feed".to_owned(),
            "(p)revious, (n)ext, (f)ind, (r)efresh, (b)ack, (q)uit".to_owned(),
            "▶".to_owned(),
        )
    };

    clear_screen();

    loop {
        view.clear_content();
        view.update_page(Some(&finder.page_or(&page_normal)));

        let iter = finder
            .page_or(&page_normal)
            .current_page(&finder.videos_or(&videos))
            .iter()
            .enumerate()
            .map(|(i, video)| (i, video));

        iter.for_each(|(i, entry)| {
            let video = entry.get_video();
            let line = if let Some(channel) = entry.get_channel() {
                format!(
                    "{}. {}\n   {} • {} • {}\n",
                    i.to_string().green(),
                    entry.get_title_formatted(finder.query()),
                    channel,
                    time_since_formatted(video.upload),
                    time_formatted_short(video.progress_seconds)
                )
            } else {
                format!(
                    "{}. {}\n   {} • {}\n",
                    i.to_string().green(),
                    entry.get_title_formatted(finder.query()),
                    time_since_formatted(video.upload),
                    time_formatted_short(video.progress_seconds)
                )
            };
            view.add_line(line);
        });

        let page = finder.page_or_mut(&mut page_normal);

        match view.show() {
            ViewInput::Esc => {
                let should_reset = finder
                    .query()
                    .map(|query| !query.is_empty())
                    .unwrap_or(false);

                if should_reset {
                    finder.reset(&mut view)
                }
            }
            ViewInput::Char(char) => match char {
                'q' => return Message::Quit,
                'b' => return Message::Home,
                'r' => {
                    if let Some(index) = channel_index {
                        return Message::Refresh(ViewPage::ChannelFeed(
                            index,
                            Some(page_normal.current_index),
                        ));
                    } else {
                        return Message::Refresh(ViewPage::MixedFeed(Some(
                            page_normal.current_index,
                        )));
                    }
                }
                'u' => {
                    if let Some(index) = channel_index {
                        return Message::Unsubscribe(index);
                    } else {
                        view.set_error("u is not a valid option!");
                    }
                }
                'n' => {
                    finder.page_or_mut(&mut page_normal).next_page();
                    view.clear_error();
                }
                'p' => {
                    finder.page_or_mut(&mut page_normal).prev_page();
                    view.clear_error();
                }
                'm' => {
                    if let Some(channel_index) = channel_index {
                        return Message::MoreVideos(
                            channel_index,
                            ViewPage::ChannelFeed(channel_index, Some(page.current_index)),
                            page.last_index(),
                            page.current_index,
                        );
                    } else {
                        view.set_error("m is not a valid option!");
                    }
                }
                'f' => {
                    view.clear_error();
                    let Some(input) = view.show_with_input() else {
                        finder.reset(&mut view);
                        continue;
                    };

                    let filtered = videos
                        .into_iter()
                        .filter(|video| {
                            video
                                .get_video()
                                .title
                                .to_lowercase()
                                .contains(&input.to_lowercase())
                        })
                        .cloned()
                        .collect_vec();

                    finder.update(&mut view, filtered, &input.to_lowercase());
                }
                input => {
                    view.set_error(&format!("{} is not a valid option!", input));
                }
            },
            ViewInput::Num(num) => {
                let item = finder
                    .page_or(&page_normal)
                    .item_at_index(&finder.videos_or(&videos), num);

                if let Some(VideoEntry::Mixed(channel_index, video_index, _, _)) = item {
                    return Message::Play(PlayType::Existing(VideoIndex {
                        channel_index: *channel_index,
                        video_index: *video_index,
                    }));
                } else if let Some(VideoEntry::Channel(video_index, _)) = item {
                    return Message::Play(PlayType::Existing(VideoIndex {
                        channel_index: *channel_index.unwrap(),
                        video_index: *video_index,
                    }));
                } else {
                    view.set_error(&format!("{} is not a valid option!", num));
                }
            }
        }
    }
}
