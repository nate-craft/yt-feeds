use std::process::Command;

use colored::Colorize;

use crate::{
    loading::while_loading,
    view::{Error, Message, View},
    yt::{Channels, Video, VideoIndex},
};

pub fn show(channels: &Channels, index: VideoIndex, last_view: &View) -> Message {
    let channel = channels.channel(index.into()).unwrap();
    let video = channel.video(index).unwrap();

    if let Err(Error::CommandFailed) = play(video) {
        eprintln!("Could not run play command: mpv",);
    }

    match last_view {
        View::FeedChannel(channel_index) => Message::ChannelFeed(*channel_index),
        View::MixedFeed => Message::MixedFeed,
        _ => panic!(),
    }
}

fn play(video: &Video) -> Result<(), Error> {
    let title = video.title.clone();

    while_loading(
        || {
            Command::new("mpv")
                .arg("--profile=fast")
                .arg("--hwdec=vaapi")
                .arg(&video.url)
                .output()
                .map(|_| ())
                .map_err(|_| Error::CommandFailed)
        },
        move || {
            print!("{} '{}'", "loading video".green(), title.yellow());
        },
    )
}
