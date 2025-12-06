use crate::{
    data::channels::Channels,
    pages::page_feed_channel::PageFeedChannel,
    state::state::Page,
    view::view::{View, ViewBuilder},
};

impl ViewBuilder for PageFeedChannel {
    fn build(&self, page: &Page, selection: usize, channels: &Channels) -> anyhow::Result<View> {
        Ok(View::default())
    }
}
