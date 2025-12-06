use crate::state::actions::{
    Finder, Information, Lifecycle, More, Play, Refresh, Select, Subscribe, TextInput, Unsubscribe,
    WatchLater,
};

#[derive(Clone)]
pub enum RefreshType {
    Channel,
    All,
}

#[derive(Clone)]
pub struct PageRefresh {
    refresh_type: RefreshType,
}

impl Lifecycle for PageRefresh {}
impl TextInput<()> for PageRefresh {}
impl More for PageRefresh {}
impl Information for PageRefresh {}
impl Play for PageRefresh {}
impl Subscribe for PageRefresh {}
impl Unsubscribe for PageRefresh {}
impl WatchLater for PageRefresh {}
impl Refresh for PageRefresh {}
impl Select for PageRefresh {}
impl Finder for PageRefresh {}
