use std::{cmp::max, fmt::Display, ops::Deref};

use crossterm::style::{Attribute, Color, ContentStyle, Stylize};
use itertools::repeat_n;

use crate::{
    display::screen::Screen,
    view::view::{View, LEFT_PADDING},
};

const HORIZONTAL_PADDING: usize = 8;
const MIN_SIZE_HORZ: usize = 30;
const MIN_SIZE_VERT: usize = 10;

pub struct ViewContent {
    pub(super) title: String,
    pub(super) header: Option<String>,
    pub(super) options: Vec<String>,
    pub(super) horizontal_line: String,
    pub(super) page: Option<String>,
    pub(super) options_title: String,
    pub(super) error: Option<String>,
    pub(super) lines: Vec<String>,
    pub(super) selections: Vec<NumberedLine>,
    pub(super) selections_shown: usize,
}

pub struct StyledString {
    original: String,
    styled: ContentStyle,
}

pub struct NumberedLine(Vec<String>);

// Created to avoid attaching style to string attributes
impl StyledString {
    pub fn new(
        target: impl Into<String>,
        color: Option<Color>,
        attribute: Option<Attribute>,
    ) -> Self {
        Self {
            original: target.into(),
            styled: ContentStyle::new()
                .attribute(attribute.unwrap_or(Attribute::NormalIntensity))
                .with(color.unwrap_or(Color::Reset)),
        }
    }

    pub fn color(target: impl Into<String>, color: Color) -> Self {
        Self::new(target, Some(color), None)
    }
}

impl From<String> for StyledString {
    fn from(value: String) -> Self {
        Self {
            original: value.into(),
            styled: ContentStyle::new(),
        }
    }
}
impl NumberedLine {
    pub fn formatted(&self, padding: &str, selection_index: usize, line_index: usize) -> String {
        if line_index == 0 {
            let line = self
                .deref()
                .get(line_index)
                .expect("Line by index is guaranteed to exist");

            format!(
                "{}{}{}{}",
                padding,
                selection_index.to_string().green(),
                ": ".green().reset(),
                line
            )
        } else {
            let line = self
                .deref()
                .get(line_index)
                .expect("Line by index is guaranteed to exist");

            format!("{}{}", padding, line)
        }
    }
}

impl Deref for NumberedLine {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ViewContent {
    pub fn new(view: &View) -> Result<Self, String> {
        let mut rows_used = 0;
        let page_rows = Screen::rows();
        let page_columns = Screen::columns();

        if page_rows < MIN_SIZE_VERT || page_columns < MIN_SIZE_HORZ {
            return Err(format!(
                "{}",
                Self::line_centered(
                    Some(StyledString::new(
                        "Getting pretty small in here...",
                        Some(Color::Red),
                        Some(Attribute::Bold),
                    )),
                    None,
                )
            ));
        }

        let display_options = Self::centered_multiline(
            &view.options,
            HORIZONTAL_PADDING,
            Some("|"),
            Some(Color::Reset),
            Some(" "),
        );

        // 1st: show options at bottom of screen

        let display_options_title = Self::line_centered(
            Some(StyledString::new(
                " Options ",
                Some(Color::DarkYellow),
                Some(Attribute::Bold),
            )),
            Some(StyledString::new("-", Some(Color::Yellow), None)),
        );

        rows_used += display_options.len() + 2;

        let display_line = Self::line_centered(
            None,
            Some(StyledString::new("-", Some(Color::Yellow), None)),
        );

        // 2nd: show error below options
        let display_error = if let Some(err) = &view.error {
            rows_used += 1;
            Some(format!(
                "{} {}",
                "Error:".red().bold(),
                err.as_str().red().italic()
            ))
        } else {
            None
        };

        // 3rd: Show title and header at top of screen

        let display_title = Self::line_centered(
            Some(StyledString::new(
                " YT-Feeds ",
                Some(Color::DarkYellow),
                Some(Attribute::Bold),
            )),
            Some(StyledString::new("-", Some(Color::Yellow), None)),
        );

        rows_used += 3;

        let display_header = if page_rows > 4 {
            rows_used += 1;
            Some(format!("\n {}\n", view.header.as_str().bold().cyan()))
        } else {
            None
        };

        // 4th: only show content if we have space

        let mut rows_from_content = 0;

        let display_lines = view
            .lines
            .iter()
            .map_while(|line| {
                let line_split = Self::left_multiline(
                    line,
                    page_columns,
                    LEFT_PADDING.map(|pad| pad.chars().count()),
                    false,
                );
                let limit = page_rows as i32 - rows_used as i32;
                let used = (rows_from_content + line_split.len()) as i32;

                if used > limit {
                    None
                } else {
                    rows_from_content += line_split.len();
                    Some(line_split)
                }
            })
            .flatten()
            .collect::<Vec<String>>();

        rows_used += rows_from_content;

        // 5th: only show selections if we have space

        let mut rows_from_content = 0;
        let mut selections = 0;

        let display_selections = view
            .selections
            .iter()
            .map(|line| {
                let line_split = Self::left_multiline(
                    line,
                    page_columns,
                    LEFT_PADDING.map(|pad| pad.chars().count()),
                    true,
                );

                let mut used = (rows_from_content + line_split.len()) as i32;
                let limit = page_rows as i32 - rows_used as i32;
                let showing_page_num = rows_from_content + used as usize > rows_used;

                if showing_page_num {
                    used += 2;
                }

                rows_from_content += line_split.len();

                if used <= limit && selections < 10 {
                    selections += 1;
                }

                line_split
            })
            .collect::<Vec<Vec<String>>>();

        // Do not print items over limit, but still add to count for height calculations
        let display_selections = display_selections
            .into_iter()
            .skip(view.selection_start)
            .take(selections)
            .map(|selections| NumberedLine(selections))
            .collect::<Vec<NumberedLine>>();

        let display_page = if rows_from_content > rows_used {
            let page_current =
                (view.selection_start as f32 / max(selections, 1) as f32).ceil() as usize + 1;
            let pages_total =
                (view.selections.len() as f32 / max(selections, 1) as f32).ceil() as usize;

            let display_page = Self::line_centered(
                Some(StyledString::color(
                    format!("Page: [{}/{}]", page_current as usize, pages_total as usize),
                    Color::Green,
                )),
                None,
            );

            Some(display_page)
        } else {
            None
        };

        Ok(ViewContent {
            title: display_title,
            header: display_header,
            options: display_options,
            options_title: display_options_title,
            error: display_error,
            horizontal_line: display_line,
            selections: display_selections,
            lines: display_lines,
            page: display_page,
            selections_shown: selections as usize,
        })
    }

    fn line_centered(title: Option<StyledString>, border: Option<StyledString>) -> String {
        let row_width = Screen::columns();
        let adjustment = if row_width % 2 == 0 { 1 } else { 0 };

        let fill_full = repeat_n(
            border
                .as_ref()
                .map(|border| border.original.clone())
                .unwrap_or(" ".to_owned()),
            row_width,
        )
        .collect::<String>();

        if let Some(title) = title {
            let len_dash_half = max((fill_full.chars().count() as f32 / 2.0) as i32, 2);
            let len_input_half = (title.original.chars().count() as f32 / 2.0) as i32;

            let line_fill_left: String = fill_full
                .chars()
                .take(max(len_dash_half - len_input_half, 0) as usize)
                .collect();

            let line_fill_right: String = fill_full
                .chars()
                .take(max(len_dash_half - len_input_half - adjustment, 0) as usize)
                .collect();

            format!(
                "{}{}{}",
                border
                    .as_ref()
                    .map(|display| display.styled.apply(line_fill_left.as_str()).to_string())
                    .unwrap_or(line_fill_left.to_string()),
                title.styled.apply(title.original).to_string(),
                border
                    .as_ref()
                    .map(|display| display.styled.apply(line_fill_right.as_str()).to_string())
                    .unwrap_or(line_fill_right.to_string())
            )
        } else {
            format!(
                "{}",
                border
                    .map(|display| display.styled.apply(fill_full.as_str()).to_string())
                    .unwrap_or(fill_full.to_string())
            )
        }
    }

    fn left_multiline(
        original: impl Display,
        columns: usize,
        padding_left: Option<usize>,
        is_selection: bool,
    ) -> Vec<String> {
        let mut lines = Vec::new();
        let original = original.to_string();

        let mut in_control = false;
        let mut added_visible: i32 = 0;

        original.chars().for_each(|c| {
            let line_num = max(lines.len(), 1) - 1;

            let prefix_len: i32 = if line_num == 0 && is_selection { 3 } else { 0 };

            if lines.is_empty() {
                lines.push(String::with_capacity(columns));

                if let Some(padding) = padding_left {
                    added_visible = padding as i32;
                }
            }

            let line = lines
                .last_mut()
                .expect("Last line should be guaranteed to exist");

            //TODO: something very wrong here. Need to reconsider a lot

            if in_control && c == 'm' {
                in_control = false;
                line.push(c);
            } else if c.is_ascii_control() {
                in_control = true;
                line.push(c);
            } else if in_control {
                line.push(c);
            } else if c == '\n' {
                lines.push(String::with_capacity(columns));

                if let Some(padding) = padding_left {
                    added_visible = padding as i32;
                } else {
                    added_visible = 0;
                }
            } else if added_visible + prefix_len + 1 > columns as i32 {
                // Ignore spaces on line wrap
                if c != ' ' {
                    let mut new_line = String::with_capacity(columns);

                    if let Some(padding) = padding_left {
                        added_visible = padding as i32;
                    } else {
                        added_visible = 0;
                    }

                    new_line.push(c);
                    added_visible += 1;

                    lines.push(new_line);
                }
            } else {
                line.push(c);
                added_visible += 1;
            }
        });

        lines
    }

    fn centered_multiline(
        parts: &[String],
        padding: usize,
        separator: Option<&str>,
        color: Option<Color>,
        fill: Option<&str>,
    ) -> Vec<String> {
        let row_width = Screen::columns();
        let separator = separator.unwrap_or("");
        let adjustment = if row_width % 2 == 0 { 2 } else { 1 };
        let padding_full = repeat_n(fill.unwrap_or(""), row_width).collect::<String>();

        parts
            .iter()
            .fold(Vec::<String>::new(), |mut vec, option| {
                let fmt_first_long = format!("{} {} {}", separator, option, separator);
                match vec.last_mut() {
                    Some(last) => {
                        if last.len() + fmt_first_long.len() + padding >= row_width {
                            vec.push(fmt_first_long);
                        } else {
                            last.push_str(&format!(" {} |", option));
                        }
                    }
                    None => {
                        vec.push(fmt_first_long);
                    }
                };

                vec
            })
            .into_iter()
            .map(|line: String| {
                if padding_full.len() > line.len() {
                    let len_padding_half = (padding_full.len() as f32 / 2.0) as i32;
                    let len_line_half = (line.len() as f32 / 2.0) as i32;

                    let side_padding_left = padding_full
                        [0..=max(len_padding_half - len_line_half - 1, 0) as usize]
                        .to_string();
                    let side_padding_right = padding_full
                        [0..=max(len_padding_half - len_line_half - adjustment, 0) as usize]
                        .to_string();

                    format!(
                        "{}{}{}",
                        side_padding_left.as_str().green(),
                        color
                            .map(|color| line.as_str().with(color).to_string())
                            .unwrap_or(line),
                        side_padding_right.as_str().green()
                    )
                } else {
                    format!("{}", line.as_str().yellow())
                }
            })
            .collect()
    }
}
