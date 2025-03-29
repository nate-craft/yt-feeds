use std::process::Command;

use itertools::Itertools;
use serde_json::Value;

use crate::{view::Error, yt::ChannelInfo};

#[derive(Default)]
pub struct ChannelInfoAccumulator {
    id: Option<String>,
    name: Option<String>,
}

impl ChannelInfoAccumulator {
    pub fn accumulate(mut self, (key, value): (&String, &Value)) -> ChannelInfoAccumulator {
        let key = key.as_str();
        if key.eq("channel_id") {
            self.id = Some(value.as_str().unwrap().to_owned())
        } else if key.eq("channel") {
            self.name = Some(value.as_str().unwrap().to_owned())
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
            let option = json
                .as_object()
                .expect("JSON is not object")
                .iter()
                .fold(
                    ChannelInfoAccumulator::default(),
                    ChannelInfoAccumulator::accumulate,
                )
                .try_into()
                .ok();

            option
        })
        .unique()
        .collect()
}
