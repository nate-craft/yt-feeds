use std::cmp::{max, min};

use enum_dispatch::enum_dispatch;

use crate::{
    data::channels::Channels, display::screen::Screen, input::Input, state::state::Page,
    view::formatting::ViewContent,
};

pub(super) const LEFT_PADDING: Option<&'static str> = Some(" ");

#[enum_dispatch(ViewBuilder)]
pub trait ViewBuilder {
    fn build(&self, page: &Page, selection: usize, channels: &Channels) -> anyhow::Result<View> {
        Ok(View::default())
    }
}

pub struct View {
    pub(super) header: String,
    pub(super) options: Vec<String>,
    pub(super) lines: Vec<String>,
    pub(super) selections: Vec<String>,
    pub(super) selection_start: usize,
    pub(super) selections_per_page: usize,
    pub(super) error: Option<String>,
}

impl Default for View {
    fn default() -> Self {
        Self {
            header: Default::default(),
            selections: Default::default(),
            options: Default::default(),
            lines: Default::default(),
            error: Default::default(),
            selection_start: Default::default(),
            selections_per_page: Default::default(),
        }
    }
}

impl View {
    pub fn new(title: &str, selection_start: usize, options: Vec<String>) -> Self {
        Self {
            header: title.into(),
            lines: Vec::new(),
            selections: Vec::new(),
            selection_start,
            selections_per_page: 0,
            options,
            error: None,
        }
    }

    pub fn page_current(&self) -> usize {
        (self.selection_start as f32 / max(self.selections_per_page, 1) as f32).ceil() as usize
    }

    pub fn pages(&self) -> usize {
        (self.selections.len() as f32 / max(self.selections_per_page, 1) as f32).ceil() as usize
    }

    pub fn page_reset(&mut self) {
        self.selection_start = 0;
    }

    pub fn page_next(&mut self) {
        let page = min(self.pages() - 1, self.page_current() + 1);
        self.selection_start = page * max(self.selections_per_page, 1);
    }

    pub fn page_previous(&mut self) {
        let page = max(0, self.page_current() as i32 - 1);
        self.selection_start = page as usize * max(self.selections_per_page, 1);
    }

    pub fn selection(&self) -> usize {
        self.selection_start
    }

    pub fn set_error(&mut self, error: Option<String>) {
        self.error = error;
    }

    pub fn add_line_info(&mut self, line: impl ToString) {
        self.lines.push(line.to_string());
    }

    pub fn add_line_selection(&mut self, line: impl ToString) {
        self.selections.push(line.to_string());
    }

    pub fn display(&mut self) {
        match ViewContent::new(self) {
            Ok(content) => {
                self.selections_per_page = content.selections_shown;
                Screen::clear();
                self.display_content(&content);
            }
            Err(content) => {
                let page_rows = Screen::rows();
                Screen::clear();
                Screen::move_cursor_row((page_rows as f32 / 2.0).floor() as usize);
                println!("{}", content);
                Screen::move_cursor_row(0);
            }
        }
    }

    fn display_content(&mut self, content: &ViewContent) {
        Input::typing_enable_quiet().unwrap();
        println!("{}", content.title);

        if let Some(header) = content.header.as_ref() {
            println!("{}", header);
        }

        content
            .lines
            .iter()
            .for_each(|line| println!("{}{}", LEFT_PADDING.unwrap_or(""), line));

        content
            .selections
            .iter()
            .enumerate()
            .for_each(|(selection_index, selection)| {
                (0..selection.len()).for_each(|line_index| {
                    println!(
                        "{}",
                        selection.formatted(
                            LEFT_PADDING.unwrap_or(""),
                            selection_index,
                            line_index
                        )
                    )
                });
            });

        Screen::move_cursor_row(
            Screen::rows()
                - (content.options.len() + 3)
                - content.error.as_ref().map_or(0, |_| 1)
                - content.page.as_ref().map_or(0, |_| 1),
        );

        if let Some(page) = &content.page {
            println!("{}", page);
        }

        println!("{}", content.options_title);
        content.options.iter().for_each(|line| println!("{}", line));
        println!("{}", content.horizontal_line);

        if let Some(error) = content.error.as_ref() {
            println!("{}", error);
        }

        Input::typing_disable().unwrap();
    }
}
