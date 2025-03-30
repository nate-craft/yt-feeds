use std::io;

use colored::Colorize;
use itertools::Itertools;

use crate::{
    clear_screen,
    page::Page,
    utilities::time_since_formatted,
    view::{Message, View},
    yt::{ChannelIndex, Channels, Video, VideoIndex},
};

pub fn show_channel(channel_index: ChannelIndex, channels: &Channels) -> Message {
    let channel = channels.channel(channel_index).unwrap();

    let videos: Vec<(usize, &Video)> = channel
        .videos
        .iter()
        .enumerate()
        .sorted_by(|a, b| b.1.upload.cmp(&a.1.upload))
        .collect();

    let mut page = Page::new(10, videos.len(), 3);
    clear_screen();

    loop {
        let mut input = String::new();

        println!(
            "\n{}{}\n",
            &channel.name.cyan().bold(),
            "'s Feed".cyan().bold()
        );
        page.current_page(&videos)
            .iter()
            .enumerate()
            .map(|(i, video)| (i + page.current_index, video))
            .for_each(|(i, (_, video))| {
                if video.watched {
                    println!(
                        "{}. {}\n   {} • {}s Watched\n",
                        i.to_string().green(),
                        video.title.bright_yellow().underline(),
                        time_since_formatted(video.upload),
                        video.progress_seconds.unwrap_or(0).to_string()
                    )
                } else {
                    println!(
                        "{}. {}\n   {} • {} Watched\n",
                        i.to_string().green(),
                        video.title.yellow(),
                        time_since_formatted(video.upload),
                        video.progress_seconds.unwrap_or(0).to_string()
                    );
                }
            });
        println!(
            "{}",
            "Options: [(p)revious, (n)ext, (r)efresh, (c)hannels, (u)nsubscribe, (q)uit]"
                .green()
                .italic()
        );

        io::stdin().read_line(&mut input).unwrap();
        clear_screen();
        input = input.trim().to_owned();

        if input.eq_ignore_ascii_case("q") {
            return Message::Quit;
        } else if input.eq_ignore_ascii_case("c") {
            return Message::Home;
        } else if input.eq_ignore_ascii_case("u") {
            return Message::Unsubscribe(channel_index);
        } else if input.eq_ignore_ascii_case("r") {
            return Message::Refresh(View::FeedChannel(channel_index));
        } else if input.eq_ignore_ascii_case("n") {
            page.next_page();
        } else if input.eq_ignore_ascii_case("p") {
            page.prev_page();
        } else {
            match &input.parse::<usize>() {
                Ok(index) => {
                    if page.item_is_at_index(*index) {
                        return Message::Play(VideoIndex {
                            channel_index: channel_index.0,
                            video_index: *index,
                        });
                    } else {
                        println!("{} {}", input.red(), "is not a valid option!".red());
                    }
                }
                Err(_) => println!("{} {}", input.red(), "is not a valid option!".red()),
            };
        }
    }
}

pub fn show_mixed(channels: &Channels) -> Message {
    clear_screen();

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

    loop {
        let mut input = String::new();
        println!("\n{}\n", "Subscription Feed".cyan().bold());
        page.current_page(&videos)
            .iter()
            .enumerate()
            .map(|(i, video)| (i + page.current_index, video))
            .for_each(|(i, (_, _, channel, video))| {
                if video.watched {
                    println!(
                        "{}. {}\n   {} • {} • {}s Watched\n",
                        i.to_string().green(),
                        video.title.bright_yellow().underline(),
                        channel,
                        time_since_formatted(video.upload),
                        video.progress_seconds.unwrap_or(0).to_string()
                    )
                } else {
                    println!(
                        "{}. {}\n   {} • {} • {}s Watched\n",
                        i.to_string().green(),
                        video.title.yellow(),
                        channel,
                        time_since_formatted(video.upload),
                        video.progress_seconds.unwrap_or(0).to_string()
                    )
                }
            });

        println!(
            "{}",
            "Options: [(p)revious, (n)ext, r(efresh), c(hannels), q(uit)]"
                .green()
                .italic()
        );
        io::stdin().read_line(&mut input).unwrap();
        clear_screen();
        input = input.trim().to_owned();

        if input.eq_ignore_ascii_case("q") {
            return Message::Quit;
        } else if input.eq_ignore_ascii_case("c") {
            return Message::Home;
        } else if input.eq_ignore_ascii_case("n") {
            page.next_page();
        } else if input.eq_ignore_ascii_case("r") {
            return Message::Refresh(View::MixedFeed);
        } else if input.eq_ignore_ascii_case("p") {
            page.prev_page();
        } else {
            let Ok(index) = input.parse::<usize>() else {
                println!("{} {}", input.red(), "is not a valid option!".red());
                continue;
            };

            match page.item_at_index(&videos, index) {
                Some((channel_index, video_index, _, _)) => {
                    return Message::Play(VideoIndex {
                        channel_index: *channel_index,
                        video_index: *video_index,
                    });
                }
                None => {
                    println!("{} {}", input.red(), "is not a valid option!".red());
                }
            }
        }
    }
}
