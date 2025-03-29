use std::rc::Rc;

use crate::yt::{Channel, ChannelIndex, VideoIndex};

pub type LastView = Rc<View>;

#[derive(Clone)]
pub enum View {
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
    Refresh(View),
    Home,
}

#[derive(Debug)]
pub enum Error {
    FileBadAccess,
    CommandFailed,
    JsonError,
    ChannelParsing,
    VideoParsing,
    TomlError,
}
