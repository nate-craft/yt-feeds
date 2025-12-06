use crate::{
    data::channels::Channels,
    pages::page_watch_later::PageWatchLater,
    state::state::Page,
    view::view::{View, ViewBuilder},
};

impl ViewBuilder for PageWatchLater {
    fn build(&self, page: &Page, selection: usize, channels: &Channels) -> anyhow::Result<View> {
        Ok(View::default())
    }
}
