use std::{
    process::{Command, Stdio},
    rc::Rc,
    thread,
};

use crossterm::style::Stylize;

use crate::{
    config::Config,
    loading::cmd_while_loading,
    log,
    view::{Error, Message, ViewPage},
    yt::{Channels, Video, VideoIndex},
};

use super::{View, ViewInput};

pub fn show(
    channels: &Channels,
    index: VideoIndex,
    last_view: &ViewPage,
    config: &Config,
) -> Message {
    let channel = channels.channel(index.into()).unwrap();
    let video = channel.video(index).unwrap();

    let mut view = View::new(
        format!("\"{}\" - {}", video.title, channel.name),
        "(p)lay, (d)etach, (s)ave, (P)lay + save, (i)nformation, (b)ack, (q)uit".to_owned(),
        "▶".to_owned(),
    );

    let last_view = last_view.or_inner();
    let mut played = false;

    loop {
        view.clear_content();

        match view.show() {
            ViewInput::Esc => return Message::Quit,
            ViewInput::Char(char) => match char {
                'q' => return Message::Quit,
                'i' => return Message::Information(index, Rc::new(last_view.clone())),
                'b' => {
                    if played {
                        return Message::Played(Rc::new(last_view.clone()), index);
                    }

                    match last_view {
                        ViewPage::ChannelFeed(channel_index, last_index) => {
                            return Message::ChannelFeed(*channel_index, *last_index)
                        }
                        ViewPage::MixedFeed(last_index) => return Message::MixedFeed(*last_index),
                        _ => panic!(),
                    }
                }
                'p' => {
                    if let Err(Error::CommandFailed(e)) = play(video) {
                        view.set_error(&format!("Could not run play command: mpv.\nError: {}", e));
                    } else {
                        played = true;
                        view.clear_error();
                    }
                }
                'P' => {
                    if let Err(Error::CommandFailed(e)) = play_and_download(video, config) {
                        view.set_error(&format!("Could not play video\nError: {}", e));
                    } else {
                        played = true;
                        view.clear_error();
                    }
                }
                's' => {
                    if let Err(Error::CommandFailed(e)) = download(video, config) {
                        view.set_error(&format!("Could not run download video\nError: {}", e));
                    } else {
                        view.clear_error();
                    }
                }
                'd' => {
                    let url = video.url();
                    thread::spawn(|| {
                        Command::new("mpv")
                            .arg(url)
                            .stdout(Stdio::null())
                            .stderr(Stdio::null())
                            .spawn()
                    });
                    view.clear_error();
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

fn play_and_download(video: &Video, config: &Config) -> Result<(), Error> {
    let url = video.url();
    let path = config.saved_video_path.clone();

    thread::spawn(move || {
        if let Err(error) = Command::new("yt-dlp")
            .arg("-o")
            .arg(format!("{}%(title)s.%(ext)s", path))
            .arg(url)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        {
            log::err(error);
        }
    });

    play(video)
}

fn download(video: &Video, config: &Config) -> Result<(), Error> {
    let title = video.title.clone();
    let url = video.url();

    cmd_while_loading(
        Command::new("yt-dlp")
            .arg("-o")
            .arg(format!("{}%(title)s.%(ext)s", config.saved_video_path))
            .arg(url)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn(),
        move || {
            print!("\r\n{}\r\n\r\n", title.as_str().cyan().bold());
            print!("{} '{}'", "Downloading ".green(), title.as_str().yellow());
        },
    )
}

fn play(video: &Video) -> Result<(), Error> {
    let title = video.title.clone();
    let url = video.url();

    cmd_while_loading(
        Command::new("mpv")
            .arg(url)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn(),
        move || {
            print!("\r\n{}\r\n\r\n", title.as_str().cyan().bold());
            print!("{} '{}'", "Playing ".green(), title.as_str().yellow());
        },
    )
}
