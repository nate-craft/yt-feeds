use std::{
    fs::{self},
    ops::{Deref, DerefMut},
};

use anyhow::anyhow;

use crate::{
    data::channel::{Channel, ChannelInfo},
    log::{error_exit, error_exit_msg},
    storage::directory::{Directory, Dirs},
};

#[derive(Default)]
pub struct Channels(pub Vec<Channel>);

#[derive(Clone, Copy)]
pub struct ChannelIndex(usize);

#[derive(Clone, Copy)]
pub struct VideoIndex {
    pub channel: ChannelIndex,
    pub video: usize,
}

impl Channels {
    pub fn load() -> anyhow::Result<Self> {
        let channels_path = Directory::try_from(Dirs::Data)?.file("channels.json")?;

        if !channels_path.is_file() {
            return Ok(Self(Vec::new()));
        }

        let read_result = serde_json::from_str::<Vec<ChannelInfo>>(&fs::read_to_string(
            channels_path,
        )?)
        .map_err(|err| {
            anyhow!(err).context(
                "Could not read channels.json. Save and delete saved channel data to continue",
            )
        });

        let storage = match read_result {
            Ok(storage) => storage,
            Err(err) => {
                error_exit(err.context("Could not read channels.json"));
            }
        };

        let cached_count = storage.len();

        storage
            .into_iter()
            .map(|info| Channel::load(info))
            .try_fold(
                Vec::with_capacity(cached_count),
                |mut channels, channel| match channel {
                    Ok(channel) => {
                        channels.push(channel);
                        Ok(channels)
                    }
                    Err(e) => Err(e),
                },
            )
            .map(|vec| vec.into())
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let channels_path = Directory::try_from(Dirs::Data)?.file("channels.json")?;

        self.deref().iter().try_for_each(|channel| channel.save())?;

        let json = serde_json::to_string_pretty::<Vec<&ChannelInfo>>(
            &self.deref().iter().map(|channel| channel.info()).collect(),
        )?;

        fs::write(&channels_path, json)
            .map_err(|err| anyhow!(err).context("Could not save data for channels.json"))
    }

    pub fn channel_mut_unchecked(&mut self, channel_index: ChannelIndex) -> &mut Channel {
        match self.get_mut(*channel_index) {
            Some(channel) => return channel,
            None => error_exit_msg(&format!(
                "Could not access channel at index: {}",
                *channel_index
            )),
        }
    }

    pub fn channel_unchecked(&self, channel_index: ChannelIndex) -> &Channel {
        match self.get(*channel_index) {
            Some(channel) => return channel,
            None => error_exit_msg(&format!(
                "Could not access channel at index: {}",
                *channel_index
            )),
        }
    }

    pub fn channel_by_id(&self, channel_id: &str) -> Option<&Channel> {
        self.iter()
            .find(|existing| existing.info().id().eq(channel_id))
    }

    pub fn channel_by_id_mut(&mut self, channel_id: &str) -> Option<&mut Channel> {
        self.iter_mut()
            .find(|existing| existing.info().id().eq(channel_id))
    }

    pub fn has_channel(&self, channel_id: &str) -> bool {
        self.channel_by_id(channel_id).is_some()
    }
}

impl From<Vec<Channel>> for Channels {
    fn from(value: Vec<Channel>) -> Self {
        Channels(value)
    }
}

impl Deref for Channels {
    type Target = Vec<Channel>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Channels {
    fn deref_mut(&mut self) -> &mut Vec<Channel> {
        &mut self.0
    }
}

impl Deref for ChannelIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<VideoIndex> for ChannelIndex {
    fn from(value: VideoIndex) -> Self {
        ChannelIndex(value.video)
    }
}
