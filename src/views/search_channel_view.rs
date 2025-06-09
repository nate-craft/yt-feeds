use crossterm::style::Stylize;

use crate::{
    config::Config,
    loading::run_while_loading,
    page::Page,
    search::fetch_channel,
    view::{Error, Message},
    yt::{fetch_channel_feed, Channel, Channels},
};

use super::{View, ViewInput};

pub fn show(channels: &Channels, config: &Config) -> Message {
    let mut view = View::new(
        "New Subscriptions".to_owned(),
        "Esc(ape)".to_owned(),
        "Search:".to_owned(),
    );

    let mut input;

    loop {
        input = match view.show_with_input() {
            Some(string) => string,
            None => return Message::Home,
        };
        if input.is_empty() {
            view.set_error("Search query can not be empty");
        } else {
            break;
        }
    }

    let input_clone = input.clone();

    let results = run_while_loading(
        || fetch_channel(&input, 20),
        move || {
            println!("{}", "\nNew Subscriptions\n".to_string().cyan().bold());
            print!("{} {}", "Searching:".green(), input_clone.as_str().yellow());
        },
    );

    let mut page = Page::new(results.len(), 1);

    let mut view = View::new(
        "New Subscriptions".to_owned(),
        "(p)revious, (n)ext, b(ack), q(uit)".to_owned(),
        "▶".to_owned(),
    );

    loop {
        view.clear_content();
        view.update_page(Some(&page));

        page.current_page(&results)
            .iter()
            .enumerate()
            .map(|(i, channel)| (i, channel))
            .for_each(|(i, channel)| {
                view.add_line(format!(
                    "{}. {} ({})",
                    i.to_string().green(),
                    channel.name.as_str().yellow(),
                    channel.id.as_str().yellow()
                ));
            });

        match view.show() {
            ViewInput::Esc | ViewInput::Char('b') => return Message::SearchChannels,
            ViewInput::Char(char) => match char {
                'q' => return Message::Quit,
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
                let Some(channel) = page.item_at_index(&results, num) else {
                    view.set_error(&format!("{} is not a valid option!", input));
                    continue;
                };

                if channels.has_channel(&channel.id) {
                    view.set_error(&format!("You are already subscribed to {}!", channel.name));
                    continue;
                }

                let name = channel.name.clone();
                let feed = run_while_loading(
                    || fetch_channel_feed(&channel.id, config.videos_per_channel, None),
                    move || {
                        println!("{}", "\nNew Subscriptions\n".cyan().bold());
                        print!(
                            "{} {}",
                            "Downloading videos for".green(),
                            name.as_str().yellow()
                        );
                    },
                );

                match feed {
                    Ok(feed) => {
                        return Message::Subscribe(Channel::new(
                            channel.name.as_str(),
                            channel.id.as_str(),
                            feed,
                        ))
                    }
                    Err(err) => match err {
                        Error::HistoryParsing => {
                            view.set_error(&format!(
                                "{}: '{}'",
                                "Could not find videos for channel", channel.name
                            ));
                        }
                        Error::CommandFailed(e) => {
                            view.set_error(&format!(
                                "Could not load in feed for channel: '{}' with command 'yt-dlp'.\nError: {}",
                                channel.id, e
                            ));
                        }
                        _ => panic!(),
                    },
                }
            }
        }
    }
}
