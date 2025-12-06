use crate::state::{
    actions::{
        Finder, Information, Lifecycle, More, Play, Refresh, Select, Subscribe, TextInput,
        Unsubscribe, WatchLater,
    },
    state::PlaySource,
};

#[derive(Clone)]
pub struct PageVideo {
    pub(crate) source: PlaySource,
}

impl PageVideo {
    pub fn new(source: PlaySource) -> Self {
        Self { source }
    }
}

impl Lifecycle for PageVideo {}
impl TextInput<()> for PageVideo {}
impl More for PageVideo {}
impl Information for PageVideo {}
impl Play for PageVideo {}
impl Subscribe for PageVideo {}
impl Unsubscribe for PageVideo {}
impl WatchLater for PageVideo {}
impl Refresh for PageVideo {}
impl Select for PageVideo {}
impl Finder for PageVideo {}
