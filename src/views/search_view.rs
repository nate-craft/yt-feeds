use std::io::{self, Write};

use colored::Colorize;

use crate::{
    clear_screen,
    loading::while_loading,
    page::Page,
    search::fetch_channel,
    view::{Error, Message},
    yt::{feed_channel, Channel, Channels},
};

pub fn show(channels: &Channels) -> Message {
    clear_screen();
    println!("{}", "\nNew Subscriptions\n".cyan().bold());
    println!("{}", "Options: [b(ack), q(uit)]\n".green().italic());
    print!("{} ", "Search:".green());
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_owned();

    if input.eq_ignore_ascii_case("q") {
        return Message::Quit;
    } else if input.eq_ignore_ascii_case("b") {
        return Message::Home;
    }

    let input_clone = input.clone();

    let results = while_loading(
        || fetch_channel(&input, 20),
        move || {
            println!("{}", "\nNew Subscriptions\n".cyan().bold());
            print!("{} {}", "Searching:".green(), input_clone.yellow());
        },
    );

    let mut page = Page::new(10, results.len());
    clear_screen();

    loop {
        println!("{}", "\nNew Subscriptions\n".cyan().bold());

        input.clear();
        results
            .iter()
            .enumerate()
            .map(|(i, channel)| (i + page.current_index, channel))
            .for_each(|(i, channel)| {
                println!(
                    "{}. {} ({})",
                    i.to_string().green(),
                    channel.name.yellow(),
                    channel.id.yellow()
                );
            });

        println!(
            "{}",
            "\nOptions: [(p)revious, (n)ext, b(ack), q(uit)]"
                .green()
                .italic()
        );
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_owned();

        if input.eq_ignore_ascii_case("q") {
            return Message::Quit;
        } else if input.eq_ignore_ascii_case("n") {
            clear_screen();
            page.next_page();
        } else if input.eq_ignore_ascii_case("p") {
            clear_screen();
            page.prev_page();
        } else if input.eq_ignore_ascii_case("b") {
            return Message::Home;
        } else {
            let Ok(index) = input.parse::<usize>() else {
                clear_screen();
                println!("{} {}", input.red(), "is not a valid option!".red());
                continue;
            };

            let Some(channel) = page.item_at_index(&results, index) else {
                clear_screen();
                println!("{} {}", input.red(), "is not a valid option!".red());
                continue;
            };

            if channels.has_channel(&channel.id) {
                clear_screen();
                println!(
                    "{} {}{}",
                    "You are already subscribed to".red(),
                    channel.name.red(),
                    "!".red()
                );
                continue;
            }

            let name = channel.name.clone();
            let feed = while_loading(
                || feed_channel(&channel.id, 30),
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
                    Error::ChannelParsing => {
                        eprintln!(
                            "{}: '{}'",
                            "Could not find videos for channel", channel.name
                        );
                    }
                    Error::CommandFailed => {
                        eprintln!(
                            "{}: '{}' with command 'yt-dlp'",
                            "Could not load in feed for channel", channel.id,
                        );
                    }
                    _ => {}
                },
            }
        }
    }
}
