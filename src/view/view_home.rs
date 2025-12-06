use crossterm::style::Stylize;

use crate::{
    data::channels::Channels,
    pages::page_home::PageHome,
    state::state::Page,
    view::view::{View, ViewBuilder},
};

impl ViewBuilder for PageHome {
    fn build(&self, page: &Page, selection: usize, channels: &Channels) -> anyhow::Result<View> {
        let mut view = View::new(
            "Home",
            selection,
            vec![
                "[p] previous".to_string(),
                "[n] next".to_string(),
                "[f] fzf".to_string(),
                "[a] all".to_string(),
                "[w] watch later".to_string(),
                "[c] channel search".to_string(),
                "[v] video channel".to_string(),
                "[r] refresh".to_string(),
                "[q] quit".to_string(),
            ],
        );

        (0..=10).for_each(|i| {
            view.add_line_selection(format!("{}", format!("Example {} ", i).yellow()));
        });

        Ok(view)
    }
}
