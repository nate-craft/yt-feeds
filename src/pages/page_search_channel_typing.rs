use crate::{
    data::channels::ChannelIndex,
    state::{
        actions::{
            Finder, Information, Lifecycle, More, Play, Refresh, Select, Subscribe, TextInput,
            Unsubscribe, WatchLater,
        },
        search::Search,
    },
};

#[derive(Clone)]
pub struct PageSearchChannelTyping {
    search: Search,
    channel: ChannelIndex,
}

impl Lifecycle for PageSearchChannelTyping {}
impl TextInput<()> for PageSearchChannelTyping {}
impl More for PageSearchChannelTyping {}
impl Information for PageSearchChannelTyping {}
impl Play for PageSearchChannelTyping {}
impl Subscribe for PageSearchChannelTyping {}
impl Unsubscribe for PageSearchChannelTyping {}
impl WatchLater for PageSearchChannelTyping {}
impl Refresh for PageSearchChannelTyping {}
impl Select for PageSearchChannelTyping {}
impl Finder for PageSearchChannelTyping {}
