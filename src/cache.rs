use std::{
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};

use crate::{
    view::Error,
    yt::{Channel, ChannelInfo, Video},
    Channels,
};

impl TryFrom<&ChannelInfo> for Channel {
    type Error = Error;

    fn try_from(value: &ChannelInfo) -> Result<Self, Self::Error> {
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
            let json: Result<Vec<Video>, _> = serde_json::from_reader(BufReader::new(file));

            if let Ok(videos) = json {
                Ok(Channel::new(value.name.clone(), value.id.clone(), videos))
            } else {
                Err(Error::JsonError)
            }
        } else {
            Err(Error::FileBadAccess)
        }
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

pub fn data_directory() -> Result<PathBuf, Error> {
    let Some(root) = dirs::data_local_dir() else {
        return Err(Error::FileBadAccess);
    };

    let root = root.join("yt-feeds/");

    if !Path::exists(&root) {
        fs::create_dir_all(&root).map_err(|_| Error::FileBadAccess)?;
    }

    Ok(root)
}
