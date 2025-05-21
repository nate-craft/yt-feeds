use std::io::{self, Write};

use colored::Colorize;

use crate::clear_screen;

pub mod feed_view;
pub mod home_view;
pub mod information_view;
pub mod player_view;
pub mod search_view;

pub struct View {
    title: String,
    options: String,
    input: String,
    content: Vec<String>,
    error: Option<String>,
}

impl View {
    pub fn new<T: Into<String>>(title: T, options: T, input: T) -> View {
        View {
            title: title.into(),
            options: format!("Options: [{}]", options.into()),
            input: input.into(),
            content: Vec::new(),
            error: None,
        }
    }

    pub fn add_line<T: Into<String>>(&mut self, line: T) {
        self.content.push(line.into());
    }

    pub fn set_error<T: Into<String>>(&mut self, error: T) {
        self.error = Some(format!("Error: {}", error.into()));
    }

    pub fn clear_error(&mut self) {
        self.error = None;
    }

    pub fn show(&self) -> String {
        clear_screen();
        if let Some(err) = &self.error {
            println!("{}", err.red().italic());
        }
        println!("\n{}\n", self.title.cyan().bold());
        self.content.iter().for_each(|line| println!("{}", line));
        if !self.content.is_empty() {
            println!();
        }
        println!("{}\n", self.options.green().italic());
        print!("{} ", self.input);

        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        return input.trim().to_owned();
    }

    fn clear_content(&mut self) {
        self.content.clear();
    }
}
