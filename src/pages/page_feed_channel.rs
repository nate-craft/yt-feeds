use crate::{
    data::channels::{ChannelIndex, Channels},
    finder::FinderData,
    state::actions::{
        ActionResult, Finder, Information, Lifecycle, More, Play, Refresh, Select, Subscribe,
        TextInput, Unsubscribe, WatchLater,
    },
};

#[derive(Clone)]
pub struct PageFeedChannel {
    channel: ChannelIndex,
    finder: FinderData,
}

impl PageFeedChannel {
    pub fn new(channel: ChannelIndex) -> Self {
        Self {
            channel,
            finder: FinderData::default(),
        }
    }
}

impl Lifecycle for PageFeedChannel {}
impl TextInput<()> for PageFeedChannel {}
impl More for PageFeedChannel {}
impl Information for PageFeedChannel {}
impl Play for PageFeedChannel {}
impl Subscribe for PageFeedChannel {}
impl Unsubscribe for PageFeedChannel {}
impl WatchLater for PageFeedChannel {}
impl Select for PageFeedChannel {}
impl Finder for PageFeedChannel {}

impl Refresh for PageFeedChannel {
    fn refresh(&self, channels: &mut Channels) -> ActionResult {
        todo!()
    }
}
