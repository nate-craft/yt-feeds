use crate::{
    data::channels::Channels,
    pages::page_video::PageVideo,
    state::state::Page,
    view::view::{View, ViewBuilder},
};

impl ViewBuilder for PageVideo {
    fn build(&self, page: &Page, selection: usize, channels: &Channels) -> anyhow::Result<View> {
        Ok(View::default())
    }
}
