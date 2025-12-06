use enum_dispatch::enum_dispatch;

use super::actions::MixedResult;
use super::actions::{
    Information, Lifecycle, More, Play, Select, Subscribe, TextInput, Unsubscribe, WatchLater,
};
use crate::data::channel::ChannelInfo;
use crate::data::channels::ChannelIndex;
use crate::data::channels::VideoIndex;
use crate::data::video::Video;
use crate::input::TextInputType;
use crate::pages::page_feed_channel::PageFeedChannel;
use crate::pages::page_feed_mixed::PageFeedMixed;
use crate::pages::page_home::PageHome;
use crate::pages::page_playing::PagePlaying;
use crate::pages::page_refresh::PageRefresh;
use crate::pages::page_search_channel::PageSearchChannel;
use crate::pages::page_search_channel_typing::PageSearchChannelTyping;
use crate::pages::page_search_video::PageSearchVideo;
use crate::pages::page_search_video_typing::PageSearchVideoTyping;
use crate::pages::page_video::PageVideo;
use crate::pages::page_watch_later::PageWatchLater;
use crate::state::actions::{ActionResult, Finder};
use crate::view::view::{View, ViewBuilder};
use crate::{data::channels::Channels, input::Message, state::actions::Refresh};

pub struct State {
    channels: Channels,
    page: Page,
    history: Vec<Page>,
    view: View,
}

pub enum UIMessage {
    Display,
    InvalidInput,
    Move(Page),
    Quit,
}

#[derive(Clone)]
#[enum_dispatch(
    Refresh,
    Information,
    Finder,
    Play,
    Subscribe,
    Unsubscribe,
    Select,
    More,
    WatchLater,
    ViewBuilder,
    TextInput,
    Lifecycle
)]
pub enum Page {
    Home(PageHome),
    FeedMixed(PageFeedMixed),
    FeedChannel(PageFeedChannel),
    SearchVideoTyping(PageSearchVideoTyping),
    SearchVideo(PageSearchVideo),
    SearchChannel(PageSearchChannel),
    SearchChannelTyping(PageSearchChannelTyping),
    WatchLater(PageWatchLater),
    Refresh(PageRefresh),
    Video(PageVideo),
    Playing(PagePlaying),
}

#[derive(Clone)]
pub enum PlaySource {
    Standalone(Video, ChannelInfo),
    Mixed(VideoIndex),
    Channel(VideoIndex),
}

impl State {
    pub fn new() -> anyhow::Result<Self> {
        let channels = Channels::load()?;
        let page = Page::Home(PageHome);
        let view = page.build(&page, 0, &channels)?;

        Ok(Self {
            channels,
            page,
            view,
            history: Vec::default(),
        })
    }

    pub fn page(&self) -> &Page {
        &self.page
    }

    pub fn move_to(&mut self, page: Page) -> anyhow::Result<UIMessage> {
        self.page = page;
        self.view.page_reset();
        self.view_build()?;
        self.page.init(&mut self.channels, &mut self.view)
    }

    pub fn move_back(&mut self) -> anyhow::Result<UIMessage> {
        self.page = self.history.pop().unwrap_or(self.page.previous_default());
        self.view_build()?;
        self.page.init(&mut self.channels, &mut self.view)
    }

    // Validates page capabilities based on enum_dispatch crate to avoid dynamic dispatch
    pub fn handle_message(&mut self, message: Message) -> anyhow::Result<UIMessage> {
        match message {
            Message::Page(page) => self.move_to(page),
            Message::Back => self.move_back(),
            Message::FinderStart => self.page.finder_start(&mut self.channels),
            Message::FinderEscape => self.page.finder_exit(&mut self.channels),
            Message::FinderInput(text_input_type) => {
                self.page.finder_input(&mut self.channels, text_input_type)
            }
            Message::TextInput(text_input_type) => {
                if let TextInputType::Submit = text_input_type {
                    let result = self
                        .page
                        .text_input_submit(&mut self.channels, &mut self.view);

                    match result {
                        MixedResult::Process(process) => process.join(),
                        MixedResult::Action(action) => action,
                    }
                } else {
                    self.page
                        .text_input(&mut self.channels, text_input_type, &mut self.view)
                }
            }
            Message::Refresh => self.page.refresh(&mut self.channels),
            Message::Select(selection) => self.page.select(&mut self.channels, selection),
            Message::ListPrevious => {
                self.view.page_previous();
                Ok(UIMessage::Display)
            }
            Message::ListNext => {
                self.view.page_next();
                Ok(UIMessage::Display)
            }
            Message::Play => self.page.play(&mut self.channels),
            Message::PlayDetached => self.page.play(&mut self.channels),
            Message::PlaySave => self.page.play_save(&mut self.channels),
            Message::Save => self.page.save(&mut self.channels),
            Message::More => self.page.more(&mut self.channels),
            Message::WatchLaterAdd(source) => self.page.watch_later_add(&mut self.channels, source),
            Message::Information => self.page.information(&mut self.channels),
            Message::Subscribe(channel_info) => {
                self.page.subscribe(&mut self.channels, channel_info)
            }
            Message::InvalidInput => Ok(UIMessage::InvalidInput),
            Message::Quit => Ok(UIMessage::Quit),
            Message::Resize => {
                self.view.page_reset();
                Ok(UIMessage::Display)
            }
        }
    }

    pub fn view_build(&mut self) -> anyhow::Result<()> {
        self.view = self
            .page
            .build(&self.page, self.view.selection(), &self.channels)?;
        Ok(())
    }

    pub fn view_display(&mut self) {
        self.view.display();
    }

    pub fn typing_enabled(&self) -> bool {
        self.page.text_is_active() || self.page.finder_active()
    }

    pub fn set_view(&mut self, view: View) {
        self.view = view;
    }

    pub fn set_error(&mut self, error: Option<String>) {
        self.view.set_error(error);
    }
}

impl Page {
    // previous page in page hiearchy if no history can be found
    fn previous_default(&self) -> Page {
        match self {
            Page::Home(_) => Page::Home(PageHome),
            Page::FeedMixed(_) => Page::Home(PageHome),
            Page::FeedChannel(_) => Page::Home(PageHome),
            Page::SearchVideo(_) => Page::Home(PageHome),
            Page::SearchChannel(_) => Page::Home(PageHome),
            Page::SearchVideoTyping(_) => Page::Home(PageHome),
            Page::SearchChannelTyping(_) => Page::Home(PageHome),
            Page::WatchLater(_) => Page::Home(PageHome),
            Page::Refresh(_) => Page::Home(PageHome),
            Page::Video(play_type) => match play_type.source {
                PlaySource::Standalone(_, _) => Page::Home(PageHome),
                PlaySource::Mixed(_) => Page::FeedMixed(PageFeedMixed),
                PlaySource::Channel(video) => {
                    Page::FeedChannel(PageFeedChannel::new(video.channel))
                }
            },
            Page::Playing(page) => Page::Video(PageVideo::new(page.source.clone())),
        }
    }

    pub fn text_input_capable(&self) -> bool {
        !matches!(self, Page::Refresh(_) | Page::Video(_))
    }
}

impl PartialEq for Page {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Home(_), Self::Home(_)) => true,
            (Self::FeedMixed(_), Self::FeedMixed(_)) => true,
            (Self::FeedChannel(_), Self::FeedChannel(_)) => true,
            (Self::SearchVideoTyping(_), Self::SearchVideoTyping(_)) => true,
            (Self::SearchVideo(_), Self::SearchVideo(_)) => true,
            (Self::SearchChannel(_), Self::SearchChannel(_)) => true,
            (Self::SearchChannelTyping(_), Self::SearchChannelTyping(_)) => true,
            (Self::WatchLater(_), Self::WatchLater(_)) => true,
            (Self::Refresh(_), Self::Refresh(_)) => true,
            (Self::Video(_), Self::Video(_)) => true,
            (Self::Playing(_), Self::Playing(_)) => true,
            _ => false,
        }
    }
}
