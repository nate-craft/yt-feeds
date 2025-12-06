use crate::{
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
pub struct PageSearchVideo {
    search: Search,
    finder: FinderData,
}

impl PageSearchVideo {
    pub fn new(search: Search, finder: FinderData) -> Self {
        Self { search, finder }
    }

    pub fn search(&self) -> &Search {
        &self.search
    }

    pub fn finder(&self) -> &FinderData {
        &self.finder
    }
}

impl Lifecycle for PageSearchVideo {}
impl TextInput<()> for PageSearchVideo {}
impl More for PageSearchVideo {}
impl Information for PageSearchVideo {}
impl Play for PageSearchVideo {}
impl Subscribe for PageSearchVideo {}
impl Unsubscribe for PageSearchVideo {}
impl WatchLater for PageSearchVideo {}
impl Refresh for PageSearchVideo {}
impl Select for PageSearchVideo {}
impl Finder for PageSearchVideo {}
