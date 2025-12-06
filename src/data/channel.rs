use std::fs;

use anyhow::anyhow;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{
    data::{channels::VideoIndex, video::Video},
    storage::directory::{Directory, Dirs},
};

#[derive(Clone)]
pub struct Channel {
    info: ChannelInfo,
    videos: Vec<Video>,
}

#[derive(Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ChannelInfo {
    name: String,
    id: String,
}

impl Channel {
    pub fn new(info: ChannelInfo, videos: Vec<Video>) -> Self {
        Self {
            info,
            videos: videos.into_iter().sorted().collect(),
        }
    }

    pub fn load(info: ChannelInfo) -> anyhow::Result<Channel> {
        let id = info.id.clone();

        let channel_path = Directory::try_from(Dirs::Data)?
            .subdir("channels")?
            .file(&format!("{}.json", &id))?;

        serde_json::from_str::<Vec<Video>>(&fs::read_to_string(channel_path)?)
            .map(|videos| Channel::new(info, videos))
            .map_err(|err| {
                anyhow!(err).context(format!(
                    "Could not read channel file {}.json. Did you recently update? Save and delete data to continue", &id))
            })
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let channel_path = Directory::try_from(Dirs::Data)?
            .subdir("channels")?
            .file(&format!("{}.json", &self.info.id))?;

        let json = serde_json::to_string_pretty::<Vec<Video>>(&self.videos)?;

        fs::write(channel_path, json).map_err(|err| {
            anyhow!(err).context(format!(
                "Could not save data for channel {}. Did you recently update? Save and delete data to continue", &self.info.id))
        })
    }

    pub fn info(&self) -> &ChannelInfo {
        &self.info
    }

    pub fn video(&self, index: VideoIndex) -> Option<&Video> {
        self.videos.get(index.video)
    }

    pub fn video_mut(&mut self, index: VideoIndex) -> Option<&mut Video> {
        self.videos.get_mut(index.video)
    }
}

impl ChannelInfo {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn url_videos(&self) -> String {
        format!("https://www.youtube.com/channel/{}/videos", self.id)
    }
}
