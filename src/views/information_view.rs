use crossterm::style::Stylize;

use crate::{
    clear_screen,
    loading::run_while_loading,
    view::{LastView, Message},
    yt::{fetch_video_description, Channels, VideoIndex},
};

use super::{View, ViewInput};

pub fn show(channels: &Channels, index: VideoIndex, last_view: LastView) -> Message {
    let channel = channels.channel(index.into()).unwrap();
    let video = channel.video(index).unwrap();
    let title = format!("\"{}\" - {}", video.title, channel.name);

    let mut view = View::new(
        title.clone(),
        "(m)ore, (b)ack, (q)uit".to_owned(),
        "▶".to_owned(),
    );
    view.add_line(format!("{}\n", "Description:".to_string().yellow()));
    view.add_line(video.description.clone());

    loop {
        match view.show() {
            ViewInput::Char(char) => match char {
                'q' => return Message::Quit,
                'b' => return Message::Play(index),
                'm' => {
                    let title_moved = title.clone();
                    let results = run_while_loading(
                        || fetch_video_description(video),
                        move || {
                            println!("{}\n", title_moved.as_str().bold().cyan());
                            print!("{} ", "Fetching more information".green());
                        },
                    );
                    match results {
                        Ok(new_description) => {
                            view.clear_content();
                            view.add_line(format!("{}\n", "Description:".yellow()));
                            view.add_line(new_description.clone());
                            clear_screen();
                            return Message::MoreInformation(index, last_view, new_description);
                        }
                        Err(e) => {
                            view.set_error(&format!(
                                "Could not fetch more information for channel: '{}'\n. Error: {:?}",
                                channel.name, e
                            ));
                        }
                    }
                }
                input => {
                    view.set_error(&format!("{} is not a valid option!", input));
                }
            },
            ViewInput::Num(num) => {
                view.set_error(&format!("{} is not a valid option!", num));
            }
        }
    }
}
