use crate::state::actions::{
    Finder, Information, Lifecycle, More, Play, Refresh, Select, Subscribe, TextInput, Unsubscribe,
    WatchLater,
};

#[derive(Clone)]
pub struct PageWatchLater;

impl Lifecycle for PageWatchLater {}
impl TextInput<()> for PageWatchLater {}
impl More for PageWatchLater {}
impl Information for PageWatchLater {}
impl Play for PageWatchLater {}
impl Subscribe for PageWatchLater {}
impl Unsubscribe for PageWatchLater {}
impl WatchLater for PageWatchLater {}
impl Refresh for PageWatchLater {}
impl Select for PageWatchLater {}
impl Finder for PageWatchLater {}
