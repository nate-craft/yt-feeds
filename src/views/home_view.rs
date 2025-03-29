use std::io;

use colored::Colorize;

use crate::{
    clear_screen,
    page::Page,
    view::{Message, View},
    yt::{ChannelIndex, Channels},
};

pub fn show(channels: &Channels) -> Message {
    let mut page = Page::new(10, channels.len());
    clear_screen();

    loop {
        let mut input = String::new();
        println!("\n{}\n", "Channel List".cyan().bold());
        channels
            .iter()
            .enumerate()
            .map(|(i, video)| (i + page.current_index, video))
            .for_each(|(i, channel)| {
                println!("{}. {}", i.to_string().green(), channel.name.yellow(),)
            });

        println!(
            "{}",
            "\nOptions: [(p)revious, (n)ext, (s)earch, a(ll), (r)efresh, q(uit)]"
                .green()
                .italic()
        );

        io::stdin().read_line(&mut input).unwrap();
        clear_screen();
        input = input.trim().to_owned();

        if input.eq_ignore_ascii_case("q") {
            return Message::Quit;
        } else if input.eq_ignore_ascii_case("s") {
            return Message::Search;
        } else if input.eq_ignore_ascii_case("r") {
            return Message::Refresh(View::Home);
        } else if input.eq_ignore_ascii_case("a") {
            return Message::MixedFeed;
        } else if input.eq_ignore_ascii_case("n") {
            page.next_page();
        } else if input.eq_ignore_ascii_case("p") {
            page.prev_page();
        } else {
            let Ok(index) = input.parse::<usize>() else {
                println!("{} {}", input.red(), "is not a valid option!".red());
                continue;
            };

            if page.item_is_at_index(index) {
                return Message::ChannelFeed(ChannelIndex(index));
            } else {
                println!("{} {}", input.red(), "is not a valid option!".red());
            }
        }
    }
}
