use std::{fmt::Display, rc::Rc};

use crate::{
    mpv::WatchProgress,
    yt::{Channel, ChannelIndex, VideoIndex, VideoInfo, VideoWatchLater},
};

pub type LastView = Rc<ViewPage>;
pub type LastIndex = usize;
pub type VideoCount = usize;
pub type WatchLaterIndex = usize;
pub type LastSearch = Rc<(Vec<VideoInfo>, String)>;

#[derive(Clone)]
pub enum PlayType {
    Existing(VideoIndex),
    New(VideoInfo, Option<LastSearch>),
    WatchLater(WatchLaterIndex),
}

#[derive(Clone)]
pub enum ViewPage {
    Home,
    ChannelFeed(ChannelIndex, Option<LastIndex>),
    MixedFeed(Option<LastIndex>),
    SearchChannels,
    Play(PlayType, LastView),
    Refreshing(LastView),
    Information(VideoIndex, LastView),
    SearchVideos,
    WatchLater,
}

#[derive(Clone)]
pub enum Message {
    MixedFeed(Option<LastIndex>),
    ChannelFeed(ChannelIndex, Option<LastIndex>),
    Play(PlayType),
    Played(LastView, Option<VideoIndex>, Option<WatchProgress>),
    Subscribe(Channel),
    Unsubscribe(ChannelIndex),
    Information(VideoIndex, LastView),
    MoreInformation(VideoIndex, LastView, String),
    MoreVideos(ChannelIndex, ViewPage, VideoCount, LastIndex),
    Refresh(ViewPage),
    WatchLater,
    WatchLaterRemove(WatchLaterIndex),
    WatchLaterAdd(VideoWatchLater, LastView),
    SearchChannels,
    SearchVideosClean,
    SearchVideos,
    Quit,
    Home,
}

#[derive(Debug)]
pub enum Error {
    FileBadAccess,
    CommandFailed(String),
    JsonParsing,
    ChannelParsing,
    VideoParsing,
    VideoNotAvailable,
    TomlParsing,
    InternalError(String),
}

impl From<ViewPage> for Message {
    fn from(view: ViewPage) -> Self {
        match view {
            ViewPage::Home => Message::Home,
            ViewPage::ChannelFeed(channel_index, last_index) => {
                Message::ChannelFeed(channel_index, last_index)
            }
            ViewPage::MixedFeed(last_index) => Message::MixedFeed(last_index),
            ViewPage::SearchChannels => Message::SearchChannels,
            ViewPage::SearchVideos => Message::SearchVideos,
            ViewPage::Play(video_index, _) => Message::Play(video_index),
            ViewPage::Refreshing(view_page) => Message::Refresh(view_page.as_ref().clone()),
            ViewPage::WatchLater => Message::WatchLater,
            ViewPage::Information(video_index, view_page) => {
                Message::Information(video_index, view_page)
            }
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Error::FileBadAccess => "Could not access file".to_owned(),
            Error::CommandFailed(command) => format!("Could not run command: {}", command),
            Error::JsonParsing => "Could not parse JSON".to_owned(),
            Error::ChannelParsing => "Could not parse channel information from yt-dlp".to_owned(),
            Error::VideoParsing => "Could not parse video information from yt-dlp".to_owned(),
            Error::TomlParsing => "Could not load toml configuration".to_owned(),
            Error::VideoNotAvailable => "Fetched video was not available".to_owned(),
            Error::InternalError(e) => format!("Internal Error({})", e.to_string()),
        };

        write!(f, "{}", msg)
    }
}

impl ViewPage {
    pub fn or_inner(&self) -> &ViewPage {
        match self {
            ViewPage::Play(_, view_page)
            | ViewPage::Refreshing(view_page)
            | ViewPage::Information(_, view_page) => view_page.or_inner(),
            _ => self,
        }
    }
}
