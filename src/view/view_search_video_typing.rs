use crossterm::style::Stylize;

use crate::{
    data::channels::Channels,
    pages::page_search_video_typing::PageSearchVideoTyping,
    state::{actions::TextInput, state::Page},
    view::view::{View, ViewBuilder},
};

impl ViewBuilder for PageSearchVideoTyping {
    fn build(&self, page: &Page, selections: usize, channels: &Channels) -> anyhow::Result<View> {
        let mut view = View::new("Video Search", 0, vec!["[esc] back".to_string()]);
        let query = page.text();
        if let Some(query) = query {
            view.add_line_info(format!("{}{}", "Search: ".green(), query.yellow().italic()));
        } else {
            view.add_line_info("Search: ".green());
        }

        Ok(view)
    }
}
