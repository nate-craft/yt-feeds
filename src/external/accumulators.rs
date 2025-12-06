use anyhow::anyhow;
use chrono::{DateTime, Local};
use serde_json::Value;

use crate::data::{channel::ChannelInfo, video::Video};

#[derive(Default)]
pub struct VideoAccumulator {
    video_id: Option<String>,
    video_title: Option<String>,
    upload_date: Option<DateTime<Local>>,
    is_available: bool,
    is_short: bool,
    description: Option<String>,
}

#[derive(Default)]
pub struct StandaloneVideoAccumulator {
    video_id: Option<String>,
    video_name: Option<String>,
    channel_id: Option<String>,
    channel_name: Option<String>,
    upload_date: Option<DateTime<Local>>,
    is_available: bool,
    is_short: bool,
    description: Option<String>,
}

#[derive(Default)]
pub struct ChannelInfoAccumulator {
    id: Option<String>,
    name: Option<String>,
}

impl TryFrom<ChannelInfoAccumulator> for ChannelInfo {
    type Error = anyhow::Error;

    fn try_from(value: ChannelInfoAccumulator) -> anyhow::Result<Self> {
        Ok(ChannelInfo::new(
            value
                .id
                .ok_or(anyhow!("yt-dlp parsing error: channel id not available"))?,
            value
                .name
                .ok_or(anyhow!("yt-dlp parsing error: channel name not available"))?,
        ))
    }
}

impl TryFrom<StandaloneVideoAccumulator> for (Video, ChannelInfo) {
    type Error = anyhow::Error;

    fn try_from(value: StandaloneVideoAccumulator) -> anyhow::Result<Self> {
        if !value.is_available || value.is_short {
            return Err(anyhow!("yt-dlp parsing error: video is not available"));
        }

        Ok((
            Video::new(
                value.video_name.ok_or(anyhow!(
                    "yt-dlp parsing error: video title is not available"
                ))?,
                value
                    .video_id
                    .ok_or(anyhow!("yt-dlp parsing error: video id is not available"))?,
                value.description.unwrap_or("N/A".to_owned()),
                value.upload_date.ok_or(anyhow!(
                    "yt-dlp parsing error: video upload date is not available"
                ))?,
                None,
            ),
            ChannelInfo::new(
                value
                    .channel_id
                    .ok_or(anyhow!("yt-dlp parsing error: channel id is not available"))?,
                value.channel_name.ok_or(anyhow!(
                    "yt-dlp parsing error: channel nameis not available"
                ))?,
            ),
        ))
    }
}
impl TryFrom<VideoAccumulator> for Video {
    type Error = anyhow::Error;

    fn try_from(value: VideoAccumulator) -> anyhow::Result<Self> {
        if !value.is_available || value.is_short {
            return Err(anyhow!("yt-dlp parsing error: video is not available"));
        }

        Ok(Video::new(
            value.video_title.ok_or(anyhow!(
                "yt-dlp parsing error: video title is not available"
            ))?,
            value
                .video_id
                .ok_or(anyhow!("yt-dlp parsing error: video id is not available"))?,
            value.description.unwrap_or("N/A".to_owned()),
            value.upload_date.ok_or(anyhow!(
                "yt-dlp parsing error: video upload date is not available"
            ))?,
            None,
        ))
    }
}

impl ChannelInfoAccumulator {
    pub fn accumulate(mut self, (key, value): (&String, &Value)) -> ChannelInfoAccumulator {
        let key = key.as_str();
        if key.eq("channel_id") {
            self.id = value.as_str().map(|value| value.to_owned());
        } else if key.eq("channel") {
            self.name = value.as_str().map(|value| value.to_owned());
        }
        self
    }
}

impl StandaloneVideoAccumulator {
    pub fn accumulate(mut self, (key, value): (&String, &Value)) -> StandaloneVideoAccumulator {
        let key = key.as_str();
        if key.eq("channel_id") {
            self.channel_id = value.as_str().map(|value| value.to_owned());
        } else if key.eq("channel") {
            self.channel_name = value.as_str().map(|value| value.to_owned());
        } else if key.eq("id") {
            self.video_id = value.as_str().map(|value| value.to_owned());
        } else if key.eq("title") {
            self.video_name = value.as_str().map(|value| value.to_owned());
        } else if key.eq("description") {
            self.description = Some(value.as_str().unwrap_or("N/A").to_owned());
        } else if key.eq("availability") {
            self.is_available = value.is_null();
        } else if key.eq("url") {
            self.is_short = value
                .as_str()
                .map(|value| value.contains("/shorts/"))
                .unwrap_or(false)
        } else if key.eq("timestamp") {
            self.upload_date = DateTime::from_timestamp(value.as_i64().unwrap_or(0), 0)
                .map(|time| time.with_timezone(&Local));
        }
        self
    }
}

impl VideoAccumulator {
    pub fn accumulate(mut self, (key, value): (&String, &Value)) -> VideoAccumulator {
        let key = key.as_str();
        if key.eq("id") {
            self.video_id = value.as_str().map(|value| value.to_owned());
        } else if key.eq("title") {
            self.video_title = value.as_str().map(|value| value.to_owned());
        } else if key.eq("description") {
            self.description = Some(value.as_str().unwrap_or("N/A").to_owned());
        } else if key.eq("availability") {
            self.is_available = value.is_null();
        } else if key.eq("url") {
            self.is_short = value
                .as_str()
                .map(|value| value.contains("/shorts/"))
                .unwrap_or(false)
        } else if key.eq("timestamp") {
            self.upload_date = DateTime::from_timestamp(value.as_i64().unwrap_or(0), 0)
                .map(|time| time.with_timezone(&Local));
        }
        self
    }
}
