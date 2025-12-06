use crate::state::{
    actions::{
        Finder, Information, Lifecycle, More, Play, Refresh, Select, Subscribe, TextInput,
        Unsubscribe, WatchLater,
    },
    state::PlaySource,
};

#[derive(Clone)]
pub struct PagePlaying {
    pub(crate) source: PlaySource,
}

impl Lifecycle for PagePlaying {}
impl TextInput<()> for PagePlaying {}
impl More for PagePlaying {}
impl Information for PagePlaying {}
impl Play for PagePlaying {}
impl Subscribe for PagePlaying {}
impl Unsubscribe for PagePlaying {}
impl WatchLater for PagePlaying {}
impl Refresh for PagePlaying {}
impl Select for PagePlaying {}
impl Finder for PagePlaying {}
