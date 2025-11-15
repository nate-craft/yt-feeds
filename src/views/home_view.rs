use crossterm::style::Stylize;

use crate::{
    page::Page,
    view::{Message, ViewPage},
    views::View,
    yt::{ChannelIndex, Channels},
};

use super::ViewInput;

pub fn show(channels: &Channels) -> Message {
    let mut page = Page::new(channels.len(), 1);
    let mut user = whoami::username();

    if let Some(first) = user.get_mut(0..1) {
        first.make_ascii_uppercase();
    }

    if let Some(last) = user.chars().last() {
        if last == 's' {
            user.push('\'');
        } else {
            user.push_str("'s");
        }
    }

    let mut view = View::new(
        format!("{} Home", user),
        "(p)revious, (n)ext, (a)ll, (s)ubscribe, (v)ideo search, (r)efresh, (w)atch later, (q)uit"
            .to_owned(),
        "▶".to_owned(),
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
                    channel.name.as_str().yellow()
                ))
            });

        match view.show() {
            ViewInput::Esc => return Message::Quit,
            ViewInput::Char(char) => match char {
                'q' => return Message::Quit,
                's' => return Message::SearchChannels,
                'v' => return Message::SearchVideosClean,
                'w' => return Message::WatchLater,
                'a' => return Message::MixedFeed(Some(page.current_index)),
                'r' => return Message::Refresh(ViewPage::Home),
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
                    return Message::ChannelFeed(
                        ChannelIndex(num + page.current_index),
                        Some(page.current_index),
                    );
                }
            }
        }
    }
}
