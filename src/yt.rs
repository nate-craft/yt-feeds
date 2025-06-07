use core::panic;
use std::{
    ops::{Deref, DerefMut},
    process::Command,
};

use chrono::{DateTime, Local};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{cache, config::Config, log, view::Error};

#[derive(Debug, Clone)]
pub struct Channel {
    pub name: String,
    pub id: String,
    pub videos: Vec<Video>,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct ChannelInfo {
    pub id: String,
    pub name: String,
}

#[derive(Debug)]
pub struct Channels(pub Vec<Channel>);

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq, PartialOrd, Ord, Hash)]
pub struct Video {
    pub title: String,
    pub id: String,
    pub watched: bool,
    pub upload: DateTime<Local>,
    pub description: String,
    pub progress_seconds: Option<i32>,
}

#[derive(Default)]
pub struct VideoAccumulator {
    id: Option<String>,
    title: Option<String>,
    upload: Option<DateTime<Local>>,
    decription: Option<String>,
    available: bool,
}

#[derive(Clone, Copy)]
pub struct VideoIndex {
    pub channel_index: usize,
    pub video_index: usize,
}

#[derive(Clone, Copy)]
pub struct ChannelIndex(pub usize);

impl Default for Channels {
    fn default() -> Self {
        Channels(Vec::default())
    }
}

impl Channels {
    pub fn new(channels_cached: &[ChannelInfo]) -> Channels {
        let history = cache::fetch_history_all().ok();
        Channels(
            channels_cached
                .iter()
                .filter_map(|cached: &ChannelInfo| {
                    cache::load_channel(cached, history.as_ref()).ok()
                })
                .collect::<Vec<Channel>>(),
        )
    }
    pub fn channel_mut(&mut self, channel_index: ChannelIndex) -> Option<&mut Channel> {
        self.get_mut(*channel_index)
    }

    pub fn channel(&self, channel_index: ChannelIndex) -> Option<&Channel> {
        self.get(*channel_index)
    }

    pub fn channel_by_id(&self, channel_id: &str) -> Option<&Channel> {
        self.iter().find(|existing| existing.id.eq(channel_id))
    }

    pub fn channel_by_id_mut(&mut self, channel_id: &str) -> Option<&mut Channel> {
        self.iter_mut().find(|existing| existing.id.eq(channel_id))
    }

    pub fn has_channel(&self, channel_id: &str) -> bool {
        self.channel_by_id(channel_id).is_some()
    }
}

impl Deref for Channels {
    type Target = Vec<Channel>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Channels {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for ChannelIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Channel {
    pub fn new(name: impl Into<String>, id: impl Into<String>, videos: Vec<Video>) -> Channel {
        let videos = videos
            .into_iter()
            .sorted_by(|a, b| b.upload.cmp(&a.upload))
            .collect();
        Channel {
            name: name.into(),
            id: id.into(),
            videos,
        }
    }

    pub fn video_mut(&mut self, index: VideoIndex) -> Option<&mut Video> {
        self.videos.get_mut(index.video_index)
    }

    pub fn video(&self, index: VideoIndex) -> Option<&Video> {
        self.videos.get(index.video_index)
    }
}

impl Video {
    pub fn new(
        title: impl Into<String>,
        id: impl Into<String>,
        description: impl Into<String>,
        upload_date: DateTime<Local>,
    ) -> Video {
        Video {
            title: title.into(),
            id: id.into(),
            upload: upload_date,
            watched: false,
            progress_seconds: None,
            description: description.into(),
        }
    }
    pub fn watched(&mut self) {
        self.watched = true;
    }

    pub fn url(&self) -> String {
        format!("{}{}", "https://www.youtube.com/watch?v=", self.id)
    }
}

impl From<Channel> for ChannelInfo {
    fn from(value: Channel) -> Self {
        ChannelInfo {
            name: value.name,
            id: value.id,
        }
    }
}

impl From<&Channel> for ChannelInfo {
    fn from(value: &Channel) -> Self {
        ChannelInfo {
            name: value.name.clone(),
            id: value.id.clone(),
        }
    }
}

impl From<VideoIndex> for ChannelIndex {
    fn from(value: VideoIndex) -> Self {
        ChannelIndex(value.channel_index)
    }
}

impl VideoAccumulator {
    pub fn accumulate(mut self, (key, value): (&String, &Value)) -> VideoAccumulator {
        let key = key.as_str();

        if key.eq("title") {
            self.title = Some(value.as_str().unwrap().to_owned());
        } else if key.eq("id") {
            self.id = Some(value.as_str().unwrap().to_owned());
        } else if key.eq("timestamp") {
            self.upload = Some(
                DateTime::from_timestamp(value.as_i64().unwrap_or(0), 0)
                    .unwrap()
                    .with_timezone(&Local),
            );
        } else if key.eq("availability") {
            self.available = value.is_null();
        } else if key.eq("description") {
            self.decription = Some(value.as_str().unwrap_or("N/A").to_owned());
        }
        self
    }
}

impl TryFrom<VideoAccumulator> for Video {
    type Error = Error;
    fn try_from(value: VideoAccumulator) -> Result<Self, Error> {
        if !value.available {
            return Err(Error::VideoParsing);
        }
        Ok(Video::new(
            value.title.ok_or(Error::VideoParsing)?,
            value.id.ok_or(Error::VideoParsing)?,
            value.decription.ok_or(Error::VideoParsing)?,
            value.upload.ok_or(Error::VideoParsing)?,
        ))
    }
}

pub fn fetch_channel_feed(
    channel: &str,
    count: usize,
    start: Option<usize>,
) -> Result<Vec<Video>, Error> {
    let cmd = Command::new("yt-dlp")
        // .arg(format!("-I{}", count))
        .arg("--playlist-items")
        .arg(format!(
            "{}:{}",
            start.unwrap_or(1),
            start.unwrap_or(1) + count
        ))
        .arg("--flat-playlist")
        .arg("--dump-json")
        .arg(format!(
            "https://www.youtube.com/channel/{}/videos",
            channel
        ))
        .arg("--extractor-args")
        .arg("youtubetab:approximate_date")
        .output()
        .map_err(|e| Error::CommandFailed(e.to_string()))?;

    let videos: Vec<Video> = String::from_utf8_lossy(&cmd.stdout)
        .to_string()
        .trim()
        .lines()
        .filter_map(|line| -> Option<Value> { serde_json::from_str(line).ok() })
        .filter_map(|json: Value| -> Option<Video> {
            let option = json
                .as_object()
                .expect("JSON is not object")
                .iter()
                .fold(VideoAccumulator::default(), VideoAccumulator::accumulate)
                .try_into()
                .ok();
            option
        })
        .unique()
        .collect();

    if videos.is_empty() {
        Err(Error::VideoParsing)
    } else {
        Ok(videos)
    }
}

pub fn fetch_more_videos(config: &Config, last_index: usize, channel: &mut Channel) -> bool {
    match fetch_channel_feed(&channel.id, config.video_count, Some(last_index)) {
        Ok(feed) => {
            // for new_video in feed {
            //     if !channel
            //         .videos
            //         .iter()
            //         .any(|existing_video| existing_video.id == new_video.id)
            //     {
            //         channel.videos.push(new_video);
            //     }
            // }
            //
            feed.into_iter()
                .for_each(|video| channel.videos.push(video));
            channel.videos.sort_by(|a, b| b.upload.cmp(&a.upload));
            return true;
        }
        Err(err) => match err {
            Error::VideoParsing => {
                log::err(format!(
                    "Could not add more videos for channel: '{}'",
                    channel.name
                ));
            }
            Error::CommandFailed(e) => {
                log::err(format!("Could not add in more videos for channel: '{}' with command 'yt-dlp'.\nError: {}", channel.name, e));
            }
            err => panic!("Error: {:?}", err),
        },
    }
    return false;
}

pub fn fetch_video_description(video: &Video) -> Result<String, Error> {
    let cmd = Command::new("yt-dlp")
        .arg("--dump-json")
        .arg(video.url())
        .output()
        .expect("Failed to execute yt-dlp");

    let json_raw = String::from_utf8_lossy(&cmd.stdout);
    serde_json::from_str(&json_raw)
        .map_err(|_| Error::JsonError)
        .and_then(|json: Value| {
            json["description"]
                .as_str()
                .ok_or(Error::JsonError)
                .map(|str| str.to_owned())
        })
}
