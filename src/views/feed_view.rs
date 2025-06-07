use crossterm::style::{Color, Stylize};
use itertools::Itertools;

use crate::{
    clear_screen,
    finder::Finder,
    page::Page,
    utilities::{self, time_formatted_short, time_since_formatted},
    view::{Message, ViewPage},
    yt::{ChannelIndex, Channels, Video, VideoIndex},
};

use super::{View, ViewInput};

pub fn show_channel(
    channel_index: ChannelIndex,
    channels: &Channels,
    last_index: Option<usize>,
) -> Message {
    let channel = channels.channel(channel_index).unwrap();
    let videos: Vec<(usize, &Video)> = channel.videos.iter().enumerate().collect();

    let mut finder = Finder::new(videos.len(), 3);
    let mut page_normal = Page::new(videos.len(), 3);

    let mut view = View::new(
        format!("{}'s Feed", &channel.name),
        "(p)revious, (n)ext, (m)ore, (f)ind, (r)efresh, (u)nsubscribe, (b)ack, (q)uit".to_owned(),
        "▶".to_owned(),
    );

    page_normal.current_index = last_index.unwrap_or(page_normal.current_index);

    clear_screen();

    loop {
        view.clear_content();
        view.update_page(Some(&finder.page_or(&page_normal)));

        finder
            .page_or(&page_normal)
            .current_page(&finder.videos_or(&videos))
            .iter()
            .enumerate()
            .map(|(i, video)| (i, video))
            .for_each(|(i, (_, video))| {
                let title = if video.watched {
                    utilities::format_substring(
                        &video.title,
                        finder.query(),
                        false,
                        Some(Color::Yellow),
                    )
                } else {
                    utilities::format_substring(
                        &video.title,
                        finder.query(),
                        false,
                        Some(Color::DarkYellow),
                    )
                };

                if video.watched {
                    view.add_line(format!(
                        "{}. {}\n   {} • {}\n",
                        i.to_string().green(),
                        title,
                        time_since_formatted(video.upload),
                        time_formatted_short(video.progress_seconds)
                    ));
                } else {
                    view.add_line(format!(
                        "{}. {}\n   {} • {}\n",
                        i.to_string().green(),
                        title,
                        time_since_formatted(video.upload),
                        time_formatted_short(video.progress_seconds)
                    ));
                }
            });

        let page = finder.page_or_mut(&mut page_normal);

        match view.show() {
            ViewInput::Esc => {
                finder.reset(&mut view);
            }
            ViewInput::Char(char) => match char {
                'q' => return Message::Quit,
                'b' => return Message::Home,
                'u' => return Message::Unsubscribe(channel_index),
                'r' => {
                    return Message::Refresh(ViewPage::ChannelFeed(
                        channel_index,
                        Some(page.current_index),
                    ))
                }
                'm' => {
                    return Message::MoreVideos(
                        channel_index,
                        ViewPage::ChannelFeed(channel_index, Some(page.current_index)),
                        page.last_index(),
                        page.current_index,
                    )
                }
                'n' => {
                    page.next_page();
                    view.clear_error();
                }
                'p' => {
                    page.prev_page();
                    view.clear_error();
                }
                'f' => {
                    view.clear_error();
                    let Some(input) = view.show_with_input() else {
                        finder.reset(&mut view);
                        continue;
                    };

                    let filtered = videos
                        .clone()
                        .into_iter()
                        .filter(|video| {
                            video.1.title.to_lowercase().contains(&input.to_lowercase())
                        })
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

                if let Some((video_index, _)) = item {
                    return Message::Play(VideoIndex {
                        channel_index: *channel_index,
                        video_index: *video_index,
                    });
                }
            }
        }
    }
}

pub fn show_mixed(channels: &Channels, last_index: Option<usize>) -> Message {
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

    let mut page_normal = Page::new(videos.len(), 3);
    let mut finder = Finder::new(videos.len(), 3);

    page_normal.current_index = last_index.unwrap_or(page_normal.current_index);

    let mut view = View::new(
        "Subscription Feed".to_owned(),
        "(p)revious, (n)ext, (f)ind, (r)efresh, (b)ack, (q)uit".to_owned(),
        "▶".to_owned(),
    );

    loop {
        view.clear_content();
        view.update_page(Some(&finder.page_or(&page_normal)));

        finder
            .page_or(&page_normal)
            .current_page(&finder.videos_or(&videos))
            .iter()
            .enumerate()
            .map(|(i, video)| (i, video))
            .for_each(|(i, (_, _, channel, video))| {
                let title = if video.watched {
                    utilities::format_substring(
                        &video.title,
                        finder.query(),
                        false,
                        Some(Color::Yellow),
                    )
                } else {
                    utilities::format_substring(
                        &video.title,
                        finder.query(),
                        false,
                        Some(Color::DarkYellow),
                    )
                };

                if video.watched {
                    view.add_line(format!(
                        "{}. {}\n   {} • {} • {}\n",
                        i.to_string().green(),
                        title,
                        channel,
                        time_since_formatted(video.upload),
                        time_formatted_short(video.progress_seconds)
                    ));
                } else {
                    view.add_line(format!(
                        "{}. {}\n   {} • {} • {}\n",
                        i.to_string().green(),
                        title,
                        channel,
                        time_since_formatted(video.upload),
                        time_formatted_short(video.progress_seconds)
                    ));
                }
            });

        match view.show() {
            ViewInput::Esc => {
                finder.reset(&mut view);
            }
            ViewInput::Char(char) => match char {
                'q' => return Message::Quit,
                'b' => return Message::Home,
                'r' => {
                    return Message::Refresh(ViewPage::MixedFeed(Some(page_normal.current_index)))
                }
                'n' => {
                    finder.page_or_mut(&mut page_normal).next_page();
                    view.clear_error();
                }
                'p' => {
                    finder.page_or_mut(&mut page_normal).prev_page();
                    view.clear_error();
                }
                'f' => {
                    view.clear_error();
                    let Some(input) = view.show_with_input() else {
                        finder.reset(&mut view);
                        continue;
                    };

                    let filtered = videos
                        .clone()
                        .into_iter()
                        .filter(|video| {
                            video.3.title.to_lowercase().contains(&input.to_lowercase())
                        })
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
