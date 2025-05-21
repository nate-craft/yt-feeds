use colored::Colorize;

use crate::{
    loading::run_while_loading,
    page::Page,
    search::fetch_channel,
    view::{Error, Message},
    yt::{fetch_channel_feed, Channel, Channels},
};

use super::View;

pub fn show(channels: &Channels) -> Message {
    let view = View::new("New Subscriptions", "b(ack), q(uit)", "Search:");
    let input = view.show().to_lowercase();

    if input.eq("q") {
        return Message::Quit;
    } else if input.eq("b") {
        return Message::Home;
    }

    let input_clone = input.clone();

    let results = run_while_loading(
        || fetch_channel(&input, 20),
        move || {
            println!("{}", "\nNew Subscriptions\n".cyan().bold());
            print!("{} {}", "Searching:".green(), input_clone.yellow());
        },
    );

    let mut page = Page::new(10, results.len(), 1);

    let mut view = View::new(
        "New Subscriptions",
        "(p)revious, (n)ext, b(ack), q(uit)",
        "▶",
    );

    loop {
        view.clear_content();

        results
            .iter()
            .enumerate()
            .map(|(i, channel)| (i + page.current_index, channel))
            .for_each(|(i, channel)| {
                view.add_line(format!(
                    "{}. {} ({})",
                    i.to_string().green(),
                    channel.name.yellow(),
                    channel.id.yellow()
                ));
            });

        match view.show().to_lowercase().as_str() {
            "q" => return Message::Quit,
            "b" => return Message::Search,
            "n" => {
                page.next_page();
                view.clear_error();
            }
            "p" => {
                page.prev_page();
                view.clear_error();
            }
            input => {
                let Ok(index) = input.parse::<usize>() else {
                    view.set_error(format!("{} is not a valid option!", input));
                    continue;
                };

                let Some(channel) = page.item_at_index(&results, index) else {
                    view.set_error(format!("{} is not a valid option!", input));
                    continue;
                };

                if channels.has_channel(&channel.id) {
                    view.set_error(format!("You are already subscribed to {}!", channel.name));
                    continue;
                }

                let name = channel.name.clone();
                let feed = run_while_loading(
                    || fetch_channel_feed(&channel.id, 30),
                    move || {
                        println!("{}", "\nNew Subscriptions\n".cyan().bold());
                        print!("{} {}", "Downloading videos for".green(), name.yellow());
                    },
                );

                match feed {
                    Ok(feed) => {
                        return Message::Subscribe(Channel::new(
                            channel.name.clone(),
                            channel.id.clone(),
                            feed,
                        ))
                    }
                    Err(err) => match err {
                        Error::HistoryParsing => {
                            view.set_error(format!(
                                "{}: '{}'",
                                "Could not find videos for channel", channel.name
                            ));
                        }
                        Error::CommandFailed(e) => {
                            view.set_error(format!(
                                "Could not load in feed for channel: '{}' with command 'yt-dlp'.\nError: {}",
                                channel.id, e
                            ));
                        }
                        _ => {}
                    },
                }
            }
        }
    }
}
