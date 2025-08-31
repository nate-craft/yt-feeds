use std::{
    process::{Command, Stdio},
    rc::Rc,
    thread,
};

use crossterm::style::Stylize;

use crate::{
    config::Config,
    loading::{cmd_while_loading, cmd_while_loading_with_background, run_while_loading},
    log,
    mpv::{WatchProgress, MPV_SOCKET},
    view::{Error, Message, PlayType, ViewPage},
    yt::{fetch_channel_feed, Channel, Channels, Video, VideoInfo, VideoWatchLater},
};

use super::{View, ViewInput};

pub fn show(
    channels: &Channels,
    watch_later: &[VideoWatchLater],
    play_type: &PlayType,
    last_view: &ViewPage,
    config: &Config,
) -> Message {
    let (url, title, progress_before, mut view) = match &play_type {
        PlayType::Existing(video_index) => {
            let channel = channels.channel((*video_index).into()).unwrap();
            let video = channel.video(*video_index).unwrap();
            let view = View::new(
                format!("\"{}\" - {}", video.title, channel.name),
                "(p)lay, (d)etach, (s)ave, (P)lay + save, (i)nformation, (b)ack, (q)uit".to_owned(),
                "▶".to_owned(),
            );

            (video.url(), video.title.clone(), video.progress, view)
        }
        PlayType::New(video_info, _) => {
            let view = View::new(
                format!("\"{}\" - {}", video_info.title, video_info.channel.name),
                "(p)lay, (d)etach, (s)ave, (P)lay + save, (S)ubscribe, (w)atch later, (b)ack, (q)uit".to_owned(),
                "▶".to_owned(),
            );

            (video_info.url(), video_info.title.clone(), None, view)
        }
        PlayType::WatchLater(index) => {
            let later = watch_later.get(*index).unwrap();
            let view = View::new(
                format!("\"{}\" - {}", later.video.title, later.channel.name),
                "(p)lay, (d)etach, (s)ave, (P)lay + save, (r)emove, (b)ack, (q)uit".to_owned(),
                "▶".to_owned(),
            );

            (
                later.video.url(),
                later.video.title.clone(),
                later.video.progress,
                view,
            )
        }
    };

    let last_view = last_view.or_inner();
    let mut play_progress: Option<WatchProgress> = None;

    loop {
        view.clear_content();

        match view.show() {
            ViewInput::Esc => return Message::Quit,
            ViewInput::Char(char) => match char {
                'q' => return Message::Quit,
                'i' => {
                    if let PlayType::Existing(index) = play_type {
                        return Message::Information(*index, Rc::new(last_view.clone()));
                    } else {
                        view.set_error("i is not a valid option!");
                    }
                }
                'p' => match play(&title, &url, progress_before.as_ref()) {
                    Err(Error::CommandFailed(e)) => {
                        view.set_error(&format!("Could not run play command: mpv.\nError: {}", e));
                    }
                    Ok(progress) => {
                        if let Some(progress) = progress {
                            play_progress = Some(progress);
                        }
                        view.clear_error();
                    }
                    _ => panic!(),
                },
                'P' => match play_and_download(&title, &url, config, progress_before.as_ref()) {
                    Err(Error::CommandFailed(e)) => {
                        view.set_error(&format!("Could not play video\nError: {}", e));
                    }
                    Ok(progress) => {
                        if let Some(progress) = progress {
                            play_progress = Some(progress);
                        }

                        view.clear_error();
                    }
                    _ => panic!(),
                },
                's' => {
                    if let Err(Error::CommandFailed(e)) = download(&title, &url, config) {
                        view.set_error(&format!("Could not run download video\nError: {}", e));
                    } else {
                        view.clear_error();
                    }
                }
                'd' => {
                    let url = url.to_owned();
                    thread::spawn(|| {
                        Command::new("mpv")
                            .arg(url)
                            .stdout(Stdio::null())
                            .stderr(Stdio::null())
                            .spawn()
                    });
                    view.clear_error();
                }
                'b' => {
                    if let Some(progress) = play_progress {
                        if let PlayType::Existing(index) = play_type {
                            return Message::Played(
                                Rc::new(last_view.clone()),
                                Some(*index),
                                Some(progress),
                            );
                        } else {
                            return Message::Played(
                                Rc::new(last_view.clone()),
                                None,
                                Some(progress),
                            );
                        }
                    }
                    return last_view.to_owned().into();
                }
                'S' => {
                    if let PlayType::New(info, _) = play_type {
                        if let Some(result) = subscribe(&mut view, info, config) {
                            return result;
                        }
                    } else {
                        view.set_error("S is not a valid option!");
                    }
                }
                'w' => {
                    if let PlayType::New(info, _) = play_type {
                        //TODO: Add description to WatchInfo
                        let later = VideoWatchLater {
                            video: Video::new(
                                info.title.clone(),
                                info.id.clone(),
                                "N/A",
                                info.upload,
                            ),
                            channel: info.channel.clone(),
                        };
                        return Message::WatchLaterAdd(later, Rc::new(last_view.to_owned()));
                    } else {
                        view.set_error("r is not a valid option!");
                    }
                }
                'r' => {
                    if let PlayType::WatchLater(index) = play_type {
                        return Message::WatchLaterRemove(*index);
                    } else {
                        view.set_error("r is not a valid option!");
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

fn subscribe(view: &mut View, info: &VideoInfo, config: &Config) -> Option<Message> {
    let name = info.channel.name.clone();
    let feed = run_while_loading(
        || fetch_channel_feed(&info.channel.id, config.videos_per_channel, None),
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
            return Some(Message::Subscribe(Channel::new(
                info.channel.name.as_str(),
                info.channel.id.as_str(),
                feed,
            )));
        }
        Err(err) => match err {
            Error::VideoParsing => {
                view.set_error(&format!(
                    "{}: '{}'",
                    "Could not find videos for channel",
                    info.channel.name.as_str()
                ));
            }
            Error::CommandFailed(e) => {
                view.set_error(&format!(
                    "Could not load in feed for channel: '{}' with command 'yt-dlp'.\nError: {}",
                    info.channel.name, e
                ));
            }
            _ => panic!(),
        },
    }
    None
}

fn play_and_download(
    title: &str,
    url: &str,
    config: &Config,
    progress_before: Option<&WatchProgress>,
) -> Result<Option<WatchProgress>, Error> {
    let path = config.saved_video_path.clone();
    let url_clone = url.to_owned();

    thread::spawn(move || {
        if let Err(error) = Command::new("yt-dlp")
            .arg("-o")
            .arg(format!("{}%(title)s.%(ext)s", path))
            .arg(url_clone)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        {
            log::err(error);
        }
    });

    play(title, url, progress_before)
}

fn download(title: &str, url: &str, config: &Config) -> Result<(), Error> {
    let title = title.to_owned();
    let url = url.to_owned();

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

fn play(
    title: &str,
    url: &str,
    progress: Option<&WatchProgress>,
) -> Result<Option<WatchProgress>, Error> {
    let title = title.to_owned();
    let url = url.to_owned();

    cmd_while_loading_with_background(
        Command::new("mpv")
            .arg(url)
            .arg(format!("{}{}", "--input-ipc-server=", MPV_SOCKET))
            .arg(format!(
                "{}{}",
                "--start=",
                progress.map(|progress| progress.current).unwrap_or(0)
            ))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn(),
        move || {
            print!("\r\n{}\r\n\r\n", title.as_str().cyan().bold());
            print!("{} '{}'", "Playing ".green(), title.as_str().yellow());
        },
        Some(Box::new(|| WatchProgress::playing())),
    )
}
