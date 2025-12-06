use crossterm::style::Stylize;

use crate::{
    data::channels::Channels,
    pages::page_search_video::PageSearchVideo,
    state::{search::Search, state::Page},
    view::view::{View, ViewBuilder},
};

impl ViewBuilder for PageSearchVideo {
    fn build(&self, page: &Page, selection: usize, channels: &Channels) -> anyhow::Result<View> {
        let mut view = View::new(
            "Video Search",
            selection,
            vec![
                "[p] previous".to_string(),
                "[n] next".to_string(),
                "[f] fzf".to_string(),
                "[b] back".to_string(),
                "[h] home".to_string(),
                "[q] quit".to_string(),
            ],
        );

        if let Search::Video(Some(data)) = self.search() {
            data.results().iter().for_each(|(video, channel)| {
                let time_since = video.date_relative_str();

                view.add_line_selection(format!(
                    "{}\n    {} • {}\n",
                    video.title().yellow(),
                    channel.name(),
                    time_since
                ));
            });
        }

        Ok(view)
    }
}
