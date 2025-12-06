use crate::{
    data::channels::Channels,
    pages::page_feed_mixed::PageFeedMixed,
    state::state::Page,
    view::view::{View, ViewBuilder},
};

impl ViewBuilder for PageFeedMixed {
    fn build(&self, page: &Page, selection: usize, channels: &Channels) -> anyhow::Result<View> {
        Ok(View::default())
    }
}
