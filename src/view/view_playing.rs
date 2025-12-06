use crate::{
    data::channels::Channels,
    pages::page_playing::PagePlaying,
    state::state::Page,
    view::view::{View, ViewBuilder},
};

impl ViewBuilder for PagePlaying {
    fn build(&self, page: &Page, selection: usize, channels: &Channels) -> anyhow::Result<View> {
        Ok(View::default())
    }
}
