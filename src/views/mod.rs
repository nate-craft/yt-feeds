use std::io::{self, Write};

use crossterm::{
    cursor::{self},
    event::{Event, KeyCode, KeyEvent},
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
}

pub struct View {
    title: String,
    options: String,
    input: String,
    content: Vec<String>,
    error: Option<String>,
}

impl View {
    pub fn new(title: String, options: String, input: String) -> View {
        View {
            title,
            options: format!("Options: [{}]", options),
            input,
            content: Vec::new(),
            error: None,
        }
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

    pub fn show_paged(&self, page: &Page) -> ViewInput {
        self.show_inner(Some(page))
    }

    pub fn show(&self) -> ViewInput {
        self.show_inner(None)
    }

    fn show_inner(&self, page: Option<&Page>) -> ViewInput {
        clear_screen();
        if let Some(err) = &self.error {
            println!("{}", err.as_str().red().italic());
        }
        println!("\n{}\n", self.title.as_str().cyan().bold());
        self.content.iter().for_each(|line| println!("{}", line));
        if !self.content.is_empty() {
            println!();
        }
        if let Some(page) = page {
            let total = page.pages_count();
            let current = page.page_current() + 1;
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

        crossterm::terminal::enable_raw_mode().unwrap();

        loop {
            let event = crossterm::event::read().unwrap();
            if let Event::Key(KeyEvent { code, .. }) = event {
                if let KeyCode::Char(c) = code {
                    crossterm::terminal::disable_raw_mode().unwrap();
                    return match c.to_digit(10) {
                        Some(num) => ViewInput::Num(num as usize),
                        None => ViewInput::Char(c),
                    };
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
        println!("{}\n", self.options.as_str().green().italic());
        print!("{} ", self.input);

        io::stdout().flush().unwrap();

        crossterm::terminal::enable_raw_mode().unwrap();
        let mut input = String::new();

        loop {
            let event = crossterm::event::read().unwrap();
            if let Event::Key(KeyEvent { code, .. }) = event {
                match code {
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
                        crossterm::terminal::disable_raw_mode().unwrap();
                        return None;
                    }
                    KeyCode::Enter => {
                        crossterm::terminal::disable_raw_mode().unwrap();
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
