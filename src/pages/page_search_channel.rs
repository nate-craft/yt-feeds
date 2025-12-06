use crate::{
    data::channel::ChannelInfo,
    finder::FinderData,
    state::{
        actions::{
            Finder, Information, Lifecycle, More, Play, Refresh, Select, Subscribe, TextInput,
            Unsubscribe, WatchLater,
        },
        search::Search,
    },
};

#[derive(Clone)]
pub struct PageSearchChannel {
    search: Search,
    finder: FinderData,
    pub(crate) channel: ChannelInfo,
}

impl Lifecycle for PageSearchChannel {}
impl TextInput<()> for PageSearchChannel {}
impl More for PageSearchChannel {}
impl Information for PageSearchChannel {}
impl Play for PageSearchChannel {}
impl Subscribe for PageSearchChannel {}
impl Unsubscribe for PageSearchChannel {}
impl WatchLater for PageSearchChannel {}
impl Refresh for PageSearchChannel {}
impl Select for PageSearchChannel {}
impl Finder for PageSearchChannel {}
