use crossterm::style::Stylize;

use crate::{
    config::Config,
    loading::run_while_loading,
    page::Page,
    search::fetch_videos,
    utilities::time_since_formatted,
    view::{Message, PlayType},
};

use super::{View, ViewInput};

pub fn show(config: &Config) -> Message {
    let mut view = View::new(
        "Video Search".to_owned(),
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

    let mut results = run_while_loading(
        || fetch_videos(&input, config.videos_per_search),
        move || {
            println!("{}", "\nVideo Search\n".to_string().cyan().bold());
            print!("{} {}", "Searching:".green(), input_clone.as_str().yellow());
        },
    );

    let mut page = Page::new(results.len(), 1);

    let mut view = View::new(
        format!("Videos for '{}'", &input),
        "(p)revious, (n)ext, b(ack), q(uit)".to_owned(),
        "▶".to_owned(),
    );

    loop {
        view.clear_content();
        view.update_page(Some(&page));

        page.current_page(&results)
            .iter()
            .enumerate()
            .map(|(i, video)| (i, video))
            .for_each(|(i, video)| {
                view.add_line(format!(
                    "{}. {}\n   {} • {}\n",
                    i.to_string().green(),
                    video.title.as_str().dark_yellow(),
                    video.channel.name,
                    time_since_formatted(video.upload),
                ));
            });

        match view.show() {
            ViewInput::Esc => return Message::Quit,
            ViewInput::Char(char) => match char {
                'q' => return Message::Quit,
                'b' => return Message::SearchVideos,
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
                let Some(video) = page.take_item_at_index(&mut results, num) else {
                    view.set_error(&format!("{} is not a valid option!", input));
                    continue;
                };

                return Message::Play(PlayType::New(video));
            }
        }
    }
}
