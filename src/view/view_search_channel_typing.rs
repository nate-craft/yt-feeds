use crate::{
    data::channels::Channels,
    pages::page_search_channel_typing::PageSearchChannelTyping,
    state::state::Page,
    view::view::{View, ViewBuilder},
};

impl ViewBuilder for PageSearchChannelTyping {
    fn build(&self, page: &Page, selection: usize, channels: &Channels) -> anyhow::Result<View> {
        let mut view = View::new(
            "Search Results",
            selection,
            vec![
                "[b] back".to_string(),
                "[h] home".to_string(),
                "[q] quit".to_string(),
            ],
        );

        Ok(view)
    }
}
