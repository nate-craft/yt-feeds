use crate::{
    data::{channel::ChannelInfo, channels::Channels, video::Video},
    external::yt,
    input::{Input, TextInputType},
    process::Process,
    state::{
        actions::{
            ActionResult, Finder, Information, Lifecycle, MixedResult, More, Play, Refresh, Select,
            Subscribe, TextInput, Unsubscribe, WatchLater,
        },
        search::{Search, SearchData},
        state::UIMessage,
    },
    view::view::View,
};

#[derive(Clone)]
pub struct PageSearchVideoTyping {
    search: Search,
}

impl PageSearchVideoTyping {
    pub fn new(search: Search) -> Self {
        Self { search }
    }
}

impl TextInput<Vec<(Video, ChannelInfo)>> for PageSearchVideoTyping {
    fn text_input_start(&mut self, channels: &mut Channels, view: &mut View) -> ActionResult {
        self.search = Search::Video(Some(SearchData::new()));
        Ok(UIMessage::Display)
    }

    fn text_input_exit(&mut self, channels: &mut Channels, view: &mut View) -> ActionResult {
        self.search = Search::Video(None);
        Input::typing_disable().map(|_| UIMessage::Display)
    }

    fn text_input_submit(
        &mut self,
        channels: &mut Channels,
        view: &mut View,
    ) -> MixedResult<Vec<(Video, ChannelInfo)>> {
        const VIDEOS: usize = 40;

        if let Some(query) = self.search.query() {
            let query = query.to_owned();
            let process = Process::background(move || yt::fetch_videos(&query, VIDEOS));

            //TODO: need to handle this result and do the below move handle somehow
            return MixedResult::Process(process);

            // match yt::fetch_videos(query, VIDEOS) {
            //     Ok(videos) => {
            //         if let Search::Video(Some(data)) = &self.search {
            //             return Ok(UIMessage::Move(Page::SearchVideo(PageSearchVideo::new(
            //                 Search::Video(Some(data.with_results(videos))),
            //                 FinderData::default(),
            //             ))));
            //         }
            //     }
            //     Err(err) => return Err(err),
            // }
        }

        MixedResult::Action(Ok(UIMessage::InvalidInput))
    }

    fn text_input(
        &mut self,
        channels: &mut Channels,
        input: TextInputType,
        view: &mut View,
    ) -> ActionResult {
        if let Search::Video(Some(data)) = &mut self.search {
            data.input(input);
        }

        Ok(UIMessage::Display)
    }

    fn text_is_active(&self) -> bool {
        match &self.search {
            Search::Video(Some(_)) => true,
            _ => false,
        }
    }

    fn text(&self) -> Option<&str> {
        self.search.query()
    }
}

impl Lifecycle for PageSearchVideoTyping {
    fn init(&mut self, channels: &mut Channels, view: &mut View) -> ActionResult {
        self.search = Search::Video(Some(SearchData::new()));
        self.text_input_start(channels, view)
    }
}

impl More for PageSearchVideoTyping {}
impl Information for PageSearchVideoTyping {}
impl Play for PageSearchVideoTyping {}
impl Subscribe for PageSearchVideoTyping {}
impl Unsubscribe for PageSearchVideoTyping {}
impl WatchLater for PageSearchVideoTyping {}
impl Refresh for PageSearchVideoTyping {}
impl Select for PageSearchVideoTyping {}
impl Finder for PageSearchVideoTyping {}
