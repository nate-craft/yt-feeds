use crate::{
    data::channels::Channels,
    state::actions::{
        ActionResult, Finder, Information, Lifecycle, More, Play, Refresh, Select, Subscribe,
        TextInput, Unsubscribe, WatchLater,
    },
};

#[derive(Clone)]
pub struct PageFeedMixed;

impl Lifecycle for PageFeedMixed {}
impl TextInput<()> for PageFeedMixed {}
impl More for PageFeedMixed {}
impl Information for PageFeedMixed {}
impl Play for PageFeedMixed {}
impl Subscribe for PageFeedMixed {}
impl Unsubscribe for PageFeedMixed {}
impl WatchLater for PageFeedMixed {}
impl Select for PageFeedMixed {}

impl Refresh for PageFeedMixed {
    fn refresh(&self, channels: &mut Channels) -> ActionResult {
        todo!()
    }
}

impl Finder for PageFeedMixed {
    fn finder_start(&self, channels: &mut Channels) -> ActionResult {
        todo!()
    }

    fn finder_exit(&self, channels: &mut Channels) -> ActionResult {
        todo!()
    }

    fn finder_input(
        &self,
        channels: &mut Channels,
        text_input_type: crate::input::TextInputType,
    ) -> ActionResult {
        todo!()
    }
}
