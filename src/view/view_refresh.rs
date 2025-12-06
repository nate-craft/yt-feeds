use crate::{
    data::channels::Channels,
    pages::page_refresh::PageRefresh,
    state::state::Page,
    view::view::{View, ViewBuilder},
};

impl ViewBuilder for PageRefresh {
    fn build(&self, page: &Page, selection: usize, channels: &Channels) -> anyhow::Result<View> {
        Ok(View::default())
    }
}
