use std::rc::Rc;

use crate::yt::{Channel, ChannelIndex, VideoIndex};

pub type LastView = Rc<ViewPage>;

#[derive(Clone)]
pub enum ViewPage {
    Home,
    FeedChannel(ChannelIndex),
    MixedFeed,
    Search,
    Play(VideoIndex, LastView),
    Refreshing(LastView),
    Information(VideoIndex, LastView),
}

#[derive(Clone)]
pub enum Message {
    MixedFeed,
    ChannelFeed(ChannelIndex),
    Play(VideoIndex),
    Subscribe(Channel),
    Unsubscribe(ChannelIndex),
    Information(VideoIndex, LastView),
    MoreInformation(VideoIndex, LastView, String),
    Search,
    Quit,
    Refresh(ViewPage),
    Home,
}

#[derive(Debug)]
pub enum Error {
    FileBadAccess,
    CommandFailed(String),
    JsonError,
    ChannelParsing,
    VideoParsing,
    TomlError,
    HistoryParsing,
}

impl ViewPage {
    pub fn or_inner(&self) -> &ViewPage {
        match self {
            ViewPage::Play(_, view_page)
            | ViewPage::Refreshing(view_page)
            | ViewPage::Information(_, view_page) => view_page,
            _ => self,
        }
    }
}
