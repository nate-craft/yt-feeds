use std::rc::Rc;

use crate::yt::{Channel, ChannelIndex, VideoIndex};

pub type LastView = Rc<ViewPage>;
pub type LastIndex = usize;

#[derive(Clone)]
pub enum ViewPage {
    Home,
    FeedChannel(ChannelIndex, Option<LastIndex>),
    MixedFeed(Option<LastIndex>),
    Search,
    Play(VideoIndex, LastView),
    Refreshing(LastView),
    Information(VideoIndex, LastView),
}

#[derive(Clone)]
pub enum Message {
    MixedFeed(Option<LastIndex>),
    ChannelFeed(ChannelIndex, Option<LastIndex>),
    Play(VideoIndex),
    Subscribe(Channel),
    Unsubscribe(ChannelIndex),
    Information(VideoIndex, LastView),
    MoreInformation(VideoIndex, LastView, String),
    MoreVideos(ChannelIndex, ViewPage, usize, LastIndex),
    Refresh(ViewPage),
    Search,
    Quit,
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

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::FileBadAccess => "Could not access file".to_owned(),
            Error::CommandFailed(command) => format!("Could not run command: {}", command),
            Error::JsonError => "Could not parse JSON".to_owned(),
            Error::ChannelParsing => "Could not parse channel information from yt-dlp".to_owned(),
            Error::VideoParsing => "Could not parse video information from yt-dlp".to_owned(),
            Error::TomlError => "Could not load toml configuration".to_owned(),
            Error::HistoryParsing => "Could not parse local MPV history".to_owned(),
        }
    }
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
