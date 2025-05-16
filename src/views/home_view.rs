use colored::Colorize;

use crate::{
    page::Page,
    view::{Message, ViewPage},
    views::View,
    yt::{ChannelIndex, Channels},
};

pub fn show(channels: &Channels) -> Message {
    let mut page = Page::new(channels.len(), channels.len(), 1);
    let user = users::get_current_username()
        .map(|user| {
            let mut user = user.to_string_lossy().to_string();
            if let Some(first) = user.get_mut(0..1) {
                first.make_ascii_uppercase();
            }
            if let Some(last) = user.chars().last() {
                if last == 's' {
                    user.push_str("'");
                } else {
                    user.push_str("'s");
                }
            }
            user
        })
        .unwrap_or("YT-Feeds".to_string());

    let mut view = View::new(
        format!("{} Home", user).as_str(),
        "(p)revious, (n)ext, (s)earch, (a)ll, (r)efresh, (q)uit",
        "🢡",
    );

    loop {
        view.clear_content();

        channels
            .iter()
            .enumerate()
            .map(|(i, video)| (i + page.current_index, video))
            .for_each(|(i, channel)| {
                view.add_line(format!(
                    "{}. {}",
                    i.to_string().green(),
                    channel.name.yellow()
                ))
            });

        match view.show().to_lowercase().as_str() {
            "q" => return Message::Quit,
            "s" => return Message::Search,
            "a" => return Message::MixedFeed,
            "r" => return Message::Refresh(ViewPage::Home),
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
                        return Message::ChannelFeed(ChannelIndex(*index));
                    }
                }
                view.set_error(format!("{} is not a valid option!", &input));
            }
        }
    }
}
