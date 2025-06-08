use std::process::Command;

use itertools::Itertools;
use serde_json::Value;

use crate::{
    view::Error,
    yt::{ChannelInfo, VideoInfo},
};

#[derive(Default)]
pub struct ChannelInfoAccumulator {
    id: Option<String>,
    name: Option<String>,
}

#[derive(Default)]
pub struct VideoInfoAccumulator {
    video_id: Option<String>,
    video_name: Option<String>,
    channel_id: Option<String>,
    channel_name: Option<String>,
    available: bool,
    is_short: bool,
}

impl ChannelInfoAccumulator {
    pub fn accumulate(mut self, (key, value): (&String, &Value)) -> ChannelInfoAccumulator {
        let key = key.as_str();
        if key.eq("channel_id") {
            self.id = Some(value.as_str().unwrap().to_owned());
        } else if key.eq("channel") {
            self.name = Some(value.as_str().unwrap().to_owned());
        }
        self
    }
}

impl TryFrom<ChannelInfoAccumulator> for ChannelInfo {
    type Error = Error;
    fn try_from(value: ChannelInfoAccumulator) -> Result<Self, Error> {
        Ok(ChannelInfo {
            id: value.id.ok_or(Error::ChannelParsing)?,
            name: value.name.ok_or(Error::ChannelParsing)?,
        })
    }
}

impl VideoInfoAccumulator {
    pub fn accumulate(mut self, (key, value): (&String, &Value)) -> VideoInfoAccumulator {
        let key = key.as_str();
        if key.eq("channel_id") {
            self.channel_id = Some(value.as_str().unwrap().to_owned());
        } else if key.eq("channel") {
            self.channel_name = Some(value.as_str().unwrap().to_owned());
        } else if key.eq("id") {
            self.video_id = Some(value.as_str().unwrap().to_owned());
        } else if key.eq("title") {
            self.video_name = Some(value.as_str().unwrap().to_owned());
        } else if key.eq("availability") {
            self.available = value.is_null();
        } else if key.eq("url") {
            self.is_short = value.as_str().unwrap().contains("/shorts/")
        }
        self
    }
}

impl TryFrom<VideoInfoAccumulator> for VideoInfo {
    type Error = Error;
    fn try_from(value: VideoInfoAccumulator) -> Result<Self, Error> {
        if !value.available || value.is_short {
            return Err(Error::VideoNotAvailable);
        }
        Ok(VideoInfo {
            id: value.video_id.ok_or(Error::VideoParsing)?,
            title: value.video_name.ok_or(Error::VideoParsing)?,
            channel: ChannelInfo {
                id: value.channel_id.ok_or(Error::VideoParsing)?,
                name: value.channel_name.ok_or(Error::VideoParsing)?,
            },
        })
    }
}

pub fn fetch_channel<'a>(channel: &str, queries: usize) -> Vec<ChannelInfo> {
    let channel_json_bytes = Command::new("yt-dlp")
        .arg("--flat-playlist")
        .arg("--dump-json")
        .arg(format!("ytsearch{}:{}", queries, channel))
        .output()
        .expect("Could not find command yt-dlp")
        .stdout;

    String::from_utf8_lossy(&channel_json_bytes)
        .to_string()
        .trim()
        .lines()
        .filter_map(|line| -> Option<Value> { serde_json::from_str(line).ok() })
        .filter_map(|json: Value| -> Option<ChannelInfo> {
            json.as_object()
                .expect("JSON is not object")
                .iter()
                .fold(
                    ChannelInfoAccumulator::default(),
                    ChannelInfoAccumulator::accumulate,
                )
                .try_into()
                .ok()
        })
        .unique()
        .collect()
}

pub fn fetch_videos<'a>(query: &str, queries: usize) -> Vec<VideoInfo> {
    let json_bytes = Command::new("yt-dlp")
        .arg("--flat-playlist")
        .arg("--dump-json")
        .arg(format!("ytsearch{}:{}", queries, query))
        .output()
        .expect("Could not find command yt-dlp")
        .stdout;

    String::from_utf8_lossy(&json_bytes)
        .to_string()
        .trim()
        .lines()
        .filter_map(|line| -> Option<Value> { serde_json::from_str(line).ok() })
        .filter_map(|json: Value| -> Option<VideoInfo> {
            json.as_object()
                .expect("JSON is not object")
                .iter()
                .fold(
                    VideoInfoAccumulator::default(),
                    VideoInfoAccumulator::accumulate,
                )
                .try_into()
                .ok()
        })
        .unique()
        .collect()
}
