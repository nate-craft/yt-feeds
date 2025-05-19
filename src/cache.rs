use crate::{
    clear_screen,
    view::Error,
    yt::{Channel, ChannelInfo, Video},
    Channels,
};

use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{BufReader, BufWriter, Read},
    path::{Path, PathBuf},
    process,
    str::FromStr,
};

pub fn load_channel(
    value: &ChannelInfo,
    history: Option<&HashMap<String, WatchProgress>>,
) -> Result<Channel, Error> {
    let Ok(root) = data_directory() else {
        eprintln!(
            "{}",
            "Could not retrieve local data directory. Caching cannot be enabled!"
        );
        return Err(Error::FileBadAccess);
    };

    if let Ok(file) = File::open(
        root.join("channels/")
            .join(format!("{}{}", &value.id, ".json")),
    ) {
        let json_videos: Result<Vec<Video>, _> = serde_json::from_reader(BufReader::new(file));

        if let Ok(mut videos) = json_videos {
            // try to apply new history
            if let Some(history) = history {
                videos.iter_mut().for_each(|video| {
                    let found = history
                        .get(&video.id)
                        .map(|history| history.progress_seconds);
                    video.progress_seconds = found;
                    if let Some(progress) = found {
                        // Never swap to false (viewer may have watched only for an instant, but that is still a watch)
                        video.watched = video.watched || progress > 0;
                    }
                });
            }
            Ok(Channel::new(value.name.clone(), value.id.clone(), videos))
        } else {
            Err(Error::JsonError)
        }
    } else {
        Err(Error::FileBadAccess)
    }
}

pub fn fetch_cached_channels() -> Option<Vec<ChannelInfo>> {
    let Ok(root) = data_directory() else {
        eprintln!(
            "{}",
            "Could not retrieve local data directory. Caching cannot be enabled!"
        );
        return None;
    };

    let path = root.join("channels.json");

    match File::open(&path) {
        Ok(file) => {
            let json: Result<Vec<ChannelInfo>, _> = serde_json::from_reader(BufReader::new(file));
            if let Ok(channels) = json {
                if channels.is_empty() {
                    None
                } else {
                    Some(channels)
                }
            } else {
                None
            }
        }
        Err(err) => {
            eprintln!("Could not open {:#?}\n{}", path, err);
            None
        }
    }
}

pub fn cache_videos(root: &Path, id: &str, videos: &Vec<Video>) -> Result<(), Error> {
    let root = root.join("channels/");
    if !Path::exists(&root) {
        fs::create_dir_all(&root).map_err(|_| Error::FileBadAccess)?;
    }
    if let Ok(file) = File::create(&root.join(format!("{}{}", &id, ".json")).as_path()) {
        serde_json::to_writer_pretty(BufWriter::new(file), videos).map_err(|_| Error::JsonError)
    } else {
        Err(Error::JsonError)
    }
}

pub fn cache_channels(channels: &Channels) -> Result<(), Error> {
    let Some(root) = data_directory().ok() else {
        return Err(Error::FileBadAccess);
    };

    if let Ok(file) = File::create(&root.join("channels.json")) {
        channels.iter().for_each(|channel| {
            if cache_videos(&root, &channel.id, &channel.videos).is_err() {
                eprintln!("{}", "Error on caching video!");
            }
        });

        let channels: Vec<ChannelInfo> = channels
            .iter()
            .map(|channel| ChannelInfo::from(channel))
            .collect();

        serde_json::to_writer_pretty(BufWriter::new(file), &channels).map_err(|_| Error::JsonError)
    } else {
        Err(Error::FileBadAccess)
    }
}

#[derive(Debug)]
pub struct WatchProgress {
    pub id: String,
    pub progress_seconds: i32,
}

#[derive(Default)]
struct WatchProgressAccumulator {
    id: Option<String>,
    progress: Option<i32>,
}

impl WatchProgressAccumulator {
    fn accumulate(mut self, line: &str) -> WatchProgressAccumulator {
        if line.starts_with("start") {
            self.progress = line
                .split("=")
                .last()
                .and_then(|string| string.parse::<f32>().ok().map(|i| i as i32));
        } else if line.starts_with("#") {
            self.id = line.split_once("?v=").map(|(_, id)| id.to_owned());
        }
        self
    }
}

impl TryFrom<WatchProgressAccumulator> for WatchProgress {
    type Error = Error;
    fn try_from(value: WatchProgressAccumulator) -> Result<Self, Error> {
        Ok(WatchProgress {
            id: value.id.ok_or(Error::HistoryParsing)?,
            progress_seconds: value.progress.ok_or(Error::HistoryParsing)?,
        })
    }
}

pub fn fetch_history_one(id: &str) -> Result<WatchProgress, Error> {
    let root = mpv_shared_path()?;

    let dir = root.join("mpv/").join("watch_later/");

    if !Path::exists(&dir) {
        return Err(Error::FileBadAccess);
    }

    let files = dir.read_dir().map_err(|_| Error::FileBadAccess)?;

    // Get file that matches history id from given title argument
    files
        .into_iter()
        .filter_map(|path| path.ok())
        .filter_map(|entry| File::open(entry.path()).ok())
        .find_map(|mut file| {
            let mut raw = String::new();
            file.read_to_string(&mut raw).ok();
            raw.trim()
                .lines()
                .fold(
                    WatchProgressAccumulator::default(),
                    WatchProgressAccumulator::accumulate,
                )
                .try_into()
                .ok()
                .filter(|history: &WatchProgress| Some(id) == Some(&history.id))
        })
        .ok_or(Error::HistoryParsing)
}

pub fn fetch_history_all() -> Result<HashMap<String, WatchProgress>, Error> {
    let root = mpv_shared_path()?;
    let dir = root.join("mpv/").join("watch_later/");

    if !Path::exists(&dir) {
        return Err(Error::FileBadAccess);
    }

    let files = dir.read_dir().map_err(|_| Error::FileBadAccess)?;

    Ok(files
        .into_iter()
        .filter_map(|path| path.ok())
        .filter_map(|entry| File::open(entry.path()).ok())
        .filter_map(|mut file| {
            // Will ignore some if watch history was not from a YT video played from this program
            let mut raw = String::new();
            file.read_to_string(&mut raw).ok();
            raw.trim()
                .lines()
                .fold(
                    WatchProgressAccumulator::default(),
                    WatchProgressAccumulator::accumulate,
                )
                .try_into()
                .ok()
                .map(|history: WatchProgress| (history.id.clone(), history))
        })
        .collect())
}

pub fn data_directory() -> Result<PathBuf, Error> {
    let root = dirs::data_local_dir()
        .or(dirs::data_dir())
        .ok_or(Error::FileBadAccess)?;

    let root = root.join("yt-feeds/");

    if !Path::exists(&root) {
        fs::create_dir_all(&root).map_err(|_| Error::FileBadAccess)?;
    }

    Ok(root)
}

fn mpv_shared_path() -> Result<PathBuf, Error> {
    match env::consts::OS {
        "linux" => dirs::state_dir().ok_or(Error::FileBadAccess),
        "macos" => {
            // MPV MacOS tries xdg config specifying to ~/.config/, then to ~/Library/Application Support.
            // Why? I have no idea. Not a Mac user
            // This may be the result of future bugs involving watch history not saving
            PathBuf::from_str("~/.config").or(dirs::config_local_dir().ok_or(Error::FileBadAccess))
        }
        "windows" => dirs::data_dir()
            .or(dirs::data_local_dir())
            .ok_or(Error::FileBadAccess),
        _ => {
            clear_screen();
            eprintln!("Could not find any directory for mpv. Report in github issues...");
            process::exit(1);
        }
    }
}
