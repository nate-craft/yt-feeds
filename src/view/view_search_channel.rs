use crate::{
    data::channels::Channels,
    pages::page_search_channel::PageSearchChannel,
    state::state::Page,
    view::view::{View, ViewBuilder},
};

impl ViewBuilder for PageSearchChannel {
    fn build(&self, page: &Page, selection: usize, channels: &Channels) -> anyhow::Result<View> {
        Ok(View::default())
    }
}
