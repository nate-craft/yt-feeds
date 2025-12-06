use crate::{
    data::channels::Channels,
    input::TextInputType,
    state::actions::{
        ActionResult, Finder, Information, Lifecycle, More, Play, Refresh, Select, Subscribe,
        TextInput, Unsubscribe, WatchLater,
    },
};

#[derive(Clone, Copy)]
pub struct PageHome;

impl Lifecycle for PageHome {}
impl More for PageHome {}
impl Information for PageHome {}
impl Play for PageHome {}
impl Subscribe for PageHome {}
impl Unsubscribe for PageHome {}
impl WatchLater for PageHome {}
impl TextInput<()> for PageHome {}

impl Refresh for PageHome {
    fn refresh(&self, channels: &mut Channels) -> ActionResult {
        todo!()
    }
}

impl Select for PageHome {
    fn select(&self, channels: &mut Channels, selection: usize) -> ActionResult {
        todo!()
    }

    fn previous(&self, channels: &mut Channels) -> ActionResult {
        todo!()
    }

    fn next(&self, channels: &mut Channels) -> ActionResult {
        todo!()
    }
}

impl Finder for PageHome {
    fn finder_start(&self, channels: &mut Channels) -> ActionResult {
        todo!()
    }

    fn finder_exit(&self, channels: &mut Channels) -> ActionResult {
        todo!()
    }

    fn finder_input(
        &self,
        channels: &mut Channels,
        text_input_type: TextInputType,
    ) -> ActionResult {
        todo!()
    }
}
