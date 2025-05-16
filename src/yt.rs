use std::{
    ops::{Deref, DerefMut},
    process::Command,
};

use chrono::{DateTime, Local};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{cache::WatchHistory, view::Error};

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
    pub url: String,
    pub watched: bool,
    pub upload: DateTime<Local>,
    pub progress_seconds: Option<i32>,
}

#[derive(Default)]
pub struct VideoAccumulator {
    id: Option<String>,
    title: Option<String>,
    upload: Option<DateTime<Local>>,
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
        Channels(
            channels_cached
                .iter()
                .filter_map(|cached: &ChannelInfo| cached.try_into().ok())
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

    pub fn add_history(&mut self, history: &[WatchHistory]) {
        history.iter().for_each(|history| {
            self.iter_mut().for_each(|channel| {
                channel.videos.iter_mut().for_each(|video| {
                    if video
                        .url
                        .split("/")
                        .last()
                        .map(|id| id == history.id)
                        .unwrap_or(false)
                    {
                        video.progress_seconds = Some(history.progress_seconds);
                    }
                });
            });
        });
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
    pub fn watched(&mut self) {
        self.watched = true;
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
        Ok(Video {
            title: value.title.ok_or(Error::VideoParsing)?,
            upload: value.upload.ok_or(Error::VideoParsing)?,
            url: format!(
                "{}{}",
                "https://www.youtube.com/watch?v=",
                value.id.ok_or(Error::VideoParsing)?
            ),
            watched: false,
            progress_seconds: None,
        })
    }
}

pub fn feed_channel(channel: &str, count: u32) -> Result<Vec<Video>, Error> {
    let cmd = Command::new("yt-dlp")
        .arg(format!("-I{}", count))
        .arg("--playlist-items")
        .arg(format!("1:{}", count))
        .arg("--flat-playlist")
        .arg("--dump-json")
        .arg(format!("{}{}", "https://www.youtube.com/channel/", channel))
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
