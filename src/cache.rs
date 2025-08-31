use crate::{
    log,
    view::Error,
    yt::{Channel, ChannelInfo, Video, VideoWatchLater},
    Channels,
};

use std::{
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};

pub fn load_channel(value: &ChannelInfo) -> Result<Channel, Error> {
    let Ok(root) = data_directory() else {
        log::err("Could not retrieve local data directory. Caching cannot be enabled!");
        return Err(Error::FileBadAccess);
    };

    let json_file = root
        .join("channels/")
        .join(format!("{}{}", &value.id, ".json"));

    let file = File::open(json_file).map_err(|_| Error::FileBadAccess)?;

    serde_json::from_reader(BufReader::new(file))
        .map(|videos| Channel::new(value.name.clone(), value.id.clone(), videos))
        .map_err(|_| Error::JsonParsing)
}

pub fn fetch_cached_channels() -> Option<Vec<ChannelInfo>> {
    let Ok(root) = data_directory() else {
        log::err("Could not retrieve local data directory. Caching cannot be enabled!");
        return None;
    };

    let path = root.join("channels.json");

    if let Ok(file) = File::open(&path) {
        serde_json::from_reader(BufReader::new(file))
            .ok()
            .filter(|channels: &Vec<ChannelInfo>| !channels.is_empty())
    } else {
        None
    }
}

pub fn fetch_watch_later_videos() -> Vec<VideoWatchLater> {
    let Ok(root) = data_directory() else {
        log::err("Could not retrieve local data directory. Watch later cannot be enabled!");
        return Vec::new();
    };

    let path = root.join("watch_later.json");

    if let Ok(file) = File::open(&path) {
        let videos = serde_json::from_reader(BufReader::new(file))
            .ok()
            .filter(|videos: &Vec<VideoWatchLater>| !videos.is_empty());

        match videos {
            Some(videos) => return videos,
            None => log::err(format!("Could not load json for {:?}\n", path)),
        }
    }

    Vec::new()
}

pub fn cache_videos(root: &Path, id: &str, videos: &Vec<Video>) -> Result<(), Error> {
    let root = root.join("channels/");

    if !Path::exists(&root) {
        fs::create_dir_all(&root).map_err(|_| Error::FileBadAccess)?;
    }

    if let Ok(file) = File::create(root.join(format!("{}{}", &id, ".json")).as_path()) {
        serde_json::to_writer_pretty(BufWriter::new(file), videos).map_err(|_| Error::JsonParsing)
    } else {
        Err(Error::JsonParsing)
    }
}

pub fn cache_watch_later(root: &Path, watch_later: &[VideoWatchLater]) -> Result<(), Error> {
    if let Ok(file) = File::create(root.join("watch_later.json")) {
        serde_json::to_writer_pretty(BufWriter::new(file), &watch_later)
            .map_err(|_| Error::JsonParsing)
    } else {
        Err(Error::FileBadAccess)
    }
}

pub fn cache_channels(channels: &Channels) -> Result<(), Error> {
    let Some(root) = data_directory().ok() else {
        return Err(Error::FileBadAccess);
    };

    if let Ok(file) = File::create(root.join("channels.json")) {
        channels.iter().for_each(|channel| {
            if cache_videos(&root, &channel.id, &channel.videos).is_err() {
                log::err(format!(
                    "Error on caching video for channel: {}!",
                    channel.id
                ));
            }
        });

        let channels: Vec<ChannelInfo> = channels.iter().map(ChannelInfo::from).collect();

        serde_json::to_writer_pretty(BufWriter::new(file), &channels)
            .map_err(|_| Error::JsonParsing)
    } else {
        Err(Error::FileBadAccess)
    }
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
