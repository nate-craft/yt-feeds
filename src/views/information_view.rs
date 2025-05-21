use colored::Colorize;

use crate::{
    clear_screen,
    loading::run_while_loading,
    view::{LastView, Message},
    yt::{fetch_video_description, Channels, VideoIndex},
};

use super::View;

pub fn show(channels: &Channels, index: VideoIndex, last_view: LastView) -> Message {
    let channel = channels.channel(index.into()).unwrap();
    let video = channel.video(index).unwrap();
    let title = format!("\"{}\" - {}", video.title, channel.name);

    let mut view = View::new(title.as_str(), "(m)ore, (b)ack, (q)uit", "▶");
    view.add_line(format!("{}\n", "Description:".yellow()));
    view.add_line(&video.description);

    loop {
        match view.show().to_lowercase().as_str() {
            "b" => return Message::Play(index),
            "m" => {
                let title_moved = title.clone();
                let results = run_while_loading(
                    || fetch_video_description(video),
                    move || {
                        println!("{}\n", title_moved.bold().cyan());
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
                        view.set_error(format!(
                            "Could not fetch more information for channel: '{}'\n. Error: {:?}",
                            channel.name, e
                        ));
                    }
                }
            }
            "q" => return Message::Quit,
            input => {
                view.set_error(format!("{} is not a valid option!", &input));
            }
        }
    }
}
