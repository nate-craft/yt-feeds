use crossterm::style::{Color, Stylize};
use itertools::Itertools;

use crate::{
    finder::Finder,
    page::Page,
    utilities::{highlight_query, time_formatted_short, time_since_formatted},
    view::{Message, PlayType},
    views::ViewInput,
    yt::{Video, VideoWatchLater},
};

use super::View;

fn get_title_formatted(video: &Video, query: Option<&str>) -> String {
    if video.watched {
        highlight_query(&video.title, query, Some(Color::Yellow))
    } else {
        highlight_query(&video.title, query, Some(Color::DarkYellow))
    }
}

pub fn show(watch_later: &Vec<VideoWatchLater>) -> Message {
    let mut view = View::new(
        "Watch Later".to_owned(),
        "(p)revious, (n)ext, (f)ind, (b)ack, (q)uit".to_owned(),
        "▶".to_owned(),
    );

    let mut page_normal = Page::new(watch_later.len(), 3);
    let mut finder = Finder::new(watch_later.len(), 3);

    loop {
        view.clear_content();
        view.update_page(Some(&finder.page_or(&page_normal)));

        let iter = finder
            .page_or(&page_normal)
            .current_page(&finder.videos_or(&watch_later))
            .iter()
            .enumerate()
            .map(|(i, video)| (i, video));

        iter.for_each(|(i, entry)| {
            let line = format!(
                "{}. {}\n   {} • {} • {}\n",
                i.to_string().green(),
                get_title_formatted(&entry.video, finder.query()),
                entry.channel.name,
                time_since_formatted(entry.video.upload),
                time_formatted_short(entry.video.progress_seconds)
            );
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

                    let filtered = watch_later
                        .into_iter()
                        .filter(|video| {
                            video
                                .video
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
                if page.item_is_at_index(num) {
                    return Message::Play(PlayType::WatchLater(page.current_index + num));
                } else {
                    view.set_error(&format!("{} is not a valid option!", num));
                }
            }
        }
    }
}
