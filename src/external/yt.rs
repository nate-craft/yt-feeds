use std::process::Command;

use anyhow::anyhow;
use itertools::Itertools;
use serde_json::Value;

use crate::{
    data::{channel::ChannelInfo, video::Video},
    external::accumulators::{
        ChannelInfoAccumulator, StandaloneVideoAccumulator, VideoAccumulator,
    },
};

pub fn fetch_channel(query: &str, queries: usize) -> anyhow::Result<Vec<ChannelInfo>> {
    let channel_json_bytes = Command::new("yt-dlp")
        .arg("--flat-playlist")
        .arg("--dump-json")
        .arg(format!("ytsearch{}:{}", queries, query))
        .output()
        .map_err(|err| {
            anyhow!(err).context(format!("Could not search yt-dlp for query: {}", query))
        })?
        .stdout;

    Ok(String::from_utf8_lossy(&channel_json_bytes)
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
        .collect())
}

pub fn fetch_videos(query: &str, queries: usize) -> anyhow::Result<Vec<(Video, ChannelInfo)>> {
    let json_bytes = Command::new("yt-dlp")
        .arg("--flat-playlist")
        .arg("--dump-json")
        .arg("--extractor-args")
        .arg("youtubetab:approximate_date")
        .arg(format!("ytsearch{}:{}", queries, query))
        .output()
        .map_err(|err| {
            anyhow!(err).context(format!("Could not search yt-dlp for query: {}", query))
        })?
        .stdout;

    Ok(String::from_utf8_lossy(&json_bytes)
        .to_string()
        .trim()
        .lines()
        .filter_map(|line| -> Option<Value> { serde_json::from_str(line).ok() })
        .filter_map(|json: Value| -> Option<(Video, ChannelInfo)> {
            json.as_object()
                .expect("JSON is not object")
                .iter()
                .fold(
                    StandaloneVideoAccumulator::default(),
                    StandaloneVideoAccumulator::accumulate,
                )
                .try_into()
                .ok()
        })
        .unique()
        .collect())
}

pub fn fetch_channel_feed(
    channel: &ChannelInfo,
    count: usize,
    start: Option<usize>,
) -> anyhow::Result<Vec<Video>> {
    let cmd = Command::new("yt-dlp")
        .arg("--playlist-items")
        .arg(format!(
            "{}:{}",
            start.unwrap_or(1),
            start.unwrap_or(1) + count
        ))
        .arg("--flat-playlist")
        .arg("--dump-json")
        .arg(channel.url_videos())
        .arg("--extractor-args")
        .arg("youtubetab:approximate_date")
        .output()
        .map_err(|err| {
            anyhow!(err).context(format!(
                "Could not fetch channel feed for {}",
                channel.name()
            ))
        })?;

    let videos: Vec<Video> = String::from_utf8_lossy(&cmd.stdout)
        .to_string()
        .trim()
        .lines()
        .filter_map(|line| -> Option<Value> { serde_json::from_str(line).ok() })
        .filter_map(|json: Value| -> Option<Video> {
            json.as_object()
                .expect("JSON is not object")
                .iter()
                .fold(VideoAccumulator::default(), VideoAccumulator::accumulate)
                .try_into()
                .ok()
        })
        .unique()
        .collect();

    if videos.is_empty() {
        Err(anyhow!(format!(
            "Could not find any videos for channel: '{}'",
            channel.name()
        )))
    } else {
        Ok(videos)
    }
}

pub fn fetch_video_description(video: &Video) -> anyhow::Result<String> {
    let cmd = Command::new("yt-dlp")
        .arg("--dump-json")
        .arg(video.url())
        .output()
        .map_err(|err| anyhow!(err).context("Failed to download new video description"))?;

    let json_raw = String::from_utf8_lossy(&cmd.stdout);

    let json_value = serde_json::from_str::<Value>(&json_raw).map_err(|err| {
        anyhow!(err).context("Failed to parse yt-dlp json while downloading full description")
    })?;

    json_value["description"]
        .as_str()
        .ok_or(anyhow!(
            "Failed to retrieve description field from yt-dlp json"
        ))
        .map(|str| str.to_owned())
}
