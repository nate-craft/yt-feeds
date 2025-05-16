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
}

#[derive(Clone)]
pub enum Message {
    MixedFeed,
    ChannelFeed(ChannelIndex),
    Play(VideoIndex),
    Subscribe(Channel),
    Unsubscribe(ChannelIndex),
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
