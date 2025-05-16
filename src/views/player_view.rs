use std::{
    process::{Command, Stdio},
    thread,
};

use colored::Colorize;

use crate::{
    loading::process_while_loading,
    view::{Error, Message, ViewPage},
    yt::{Channels, Video, VideoIndex},
};

use super::View;

pub fn show(channels: &Channels, index: VideoIndex, last_view: &ViewPage) -> Message {
    let channel = channels.channel(index.into()).unwrap();
    let video = channel.video(index).unwrap();

    let mut view = View::new(
        format!("\"{}\" - {}", video.title, channel.name).as_str(),
        "(p)lay, (d)etach, (b)ack, (q)uit",
        "🢡",
    );

    loop {
        view.clear_content();

        match view.show().to_lowercase().as_str() {
            "q" => return Message::Quit,
            "b" => match last_view {
                ViewPage::FeedChannel(channel_index) => {
                    return Message::ChannelFeed(*channel_index)
                }
                ViewPage::MixedFeed => return Message::MixedFeed,
                _ => panic!(),
            },
            "p" => {
                if let Err(Error::CommandFailed(e)) = play(video) {
                    view.set_error(format!("Could not run play command: mpv.\nError: {}", e));
                } else {
                    view.clear_error();
                }
            }
            "d" => {
                let url = video.url.clone();
                thread::spawn(|| {
                    Command::new("mpv")
                        .arg("--profile=fast")
                        .arg(url)
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .spawn()
                });
                view.clear_error();
            }
            input => {
                view.set_error(format!("{} is not a valid option!", &input));
            }
        }
    }
}

fn play(video: &Video) -> Result<(), Error> {
    let title = video.title.clone();
    process_while_loading(
        Command::new("mpv")
            .arg("--profile=fast")
            .arg(&video.url)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn(),
        move || {
            println!("{}\n", title.cyan().bold());
            print!("{} '{}'", "Playing ".green(), title.yellow());
        },
    )
}
