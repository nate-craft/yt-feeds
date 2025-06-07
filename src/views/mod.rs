use std::io::{self, Write};

use crossterm::{
    cursor,
    event::{Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::Stylize,
    terminal::{self, ClearType},
};

use crate::{clear_screen, page::Page};

pub mod feed_view;
pub mod home_view;
pub mod information_view;
pub mod player_view;
pub mod search_view;

pub enum ViewInput {
    Char(char),
    Num(usize),
    Esc,
}

pub struct View {
    title: String,
    options: String,
    input: String,
    content: Vec<String>,
    error: Option<String>,
    pages_progress: Option<(usize, usize)>,
    filter: Option<String>,
}

impl View {
    pub fn new(title: String, options: String, input: String) -> View {
        View {
            title,
            options: format!("Options: [{}]", options),
            input,
            content: Vec::new(),
            error: None,
            pages_progress: None,
            filter: None,
        }
    }

    pub fn update_page(&mut self, page: Option<&Page>) {
        self.pages_progress = page.map(|page| (page.page_current(), page.pages_count()));
    }

    pub fn update_filter(&mut self, filter: Option<String>) {
        self.filter = filter;
    }

    pub fn add_line(&mut self, line: String) {
        self.content.push(line);
    }

    pub fn set_error(&mut self, error: &str) {
        self.error = Some(format!("Error: {}", error));
    }

    pub fn clear_error(&mut self) {
        self.error = None;
    }

    pub fn show(&self) -> ViewInput {
        clear_screen();
        if let Some(err) = &self.error {
            println!("{}", err.as_str().red().italic());
        }

        println!("\n{}\n", self.title.as_str().cyan().bold());
        self.content.iter().for_each(|line| println!("{}", line));

        if !self.content.is_empty() {
            println!();
        }

        if let Some((current, total)) = self.pages_progress {
            if total > 0 {
                println!(
                    "{}{}{}{}{}\n",
                    "Page: [".yellow(),
                    current.to_string().dark_yellow(),
                    "/".yellow(),
                    total.to_string().yellow(),
                    "]".yellow()
                )
            }
        }

        println!("{}\n", self.options.as_str().green().italic());
        if let Some(filter) = &self.filter {
            print!(
                "{} {}{}{}",
                "Filtering".green(),
                "\"".green(),
                filter.clone().yellow(),
                "\"".green()
            );
        }

        io::stdout().flush().unwrap();

        execute!(io::stdout(), cursor::Hide).unwrap();
        terminal::enable_raw_mode().unwrap();

        loop {
            let event = crossterm::event::read().unwrap();
            if let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = event
            {
                if let KeyCode::Char('c') = code {
                    if modifiers.eq(&KeyModifiers::CONTROL) {
                        execute!(io::stdout(), cursor::Show).unwrap();
                        terminal::disable_raw_mode().unwrap();
                        return ViewInput::Char('q');
                    }
                }
                if let KeyCode::Char(c) = code {
                    terminal::disable_raw_mode().unwrap();
                    execute!(io::stdout(), cursor::Show).unwrap();
                    return match c.to_digit(10) {
                        Some(num) => ViewInput::Num(num as usize),
                        None => ViewInput::Char(c),
                    };
                } else if let KeyCode::Esc = code {
                    terminal::disable_raw_mode().unwrap();
                    execute!(io::stdout(), cursor::Show).unwrap();
                    return ViewInput::Esc;
                }
            }
        }
    }

    pub fn show_with_input(&self) -> Option<String> {
        clear_screen();
        if let Some(err) = &self.error {
            println!("{}", err.as_str().red().italic());
        }

        println!("\n{}\n", self.title.as_str().cyan().bold());
        self.content.iter().for_each(|line| println!("{}", line));

        if !self.content.is_empty() {
            println!();
        }

        if let Some((current, total)) = self.pages_progress {
            if total > 0 {
                println!(
                    "{}{}{}{}{}\n",
                    "Page: [".yellow(),
                    current.to_string().dark_yellow(),
                    "/".yellow(),
                    total.to_string().yellow(),
                    "]".yellow()
                )
            }
        }

        println!("{}\n", self.options.as_str().green().italic());
        print!("{} ", self.input);

        io::stdout().flush().unwrap();

        terminal::enable_raw_mode().unwrap();
        execute!(io::stdout(), cursor::Hide).unwrap();
        let mut input = String::new();

        loop {
            let event = crossterm::event::read().unwrap();
            if let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = event
            {
                match code {
                    KeyCode::Char('c') => {
                        if modifiers.eq(&KeyModifiers::CONTROL) {
                            execute!(io::stdout(), cursor::Show).unwrap();
                            terminal::disable_raw_mode().unwrap();
                            return None;
                        }
                    }
                    KeyCode::Char(c) => {
                        input.push(c);
                        print!("{}", c);
                        io::stdout().flush().unwrap();
                    }
                    KeyCode::Backspace => {
                        if input.is_empty() {
                            continue;
                        }
                        input.remove(input.len() - 1);
                        execute!(
                            io::stdout(),
                            cursor::MoveLeft(1),
                            terminal::Clear(ClearType::FromCursorDown)
                        )
                        .unwrap();
                    }
                    KeyCode::Esc => {
                        execute!(io::stdout(), cursor::Show).unwrap();
                        terminal::disable_raw_mode().unwrap();
                        return None;
                    }
                    KeyCode::Enter => {
                        execute!(io::stdout(), cursor::Show).unwrap();
                        terminal::disable_raw_mode().unwrap();
                        return Some(input.trim().to_owned());
                    }

                    _ => {}
                }
            }
        }
    }

    pub fn clear_content(&mut self) {
        self.content.clear();
    }
}
