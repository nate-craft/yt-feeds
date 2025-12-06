use std::io;

use anyhow::anyhow;
use crossterm::{
    cursor, execute,
    terminal::{self, ClearType},
};

use crate::log;

pub struct Screen;

impl Screen {
    pub fn clear() {
        if let Err(err) = execute!(
            io::stdout(),
            terminal::Clear(ClearType::Purge),
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        ) {
            log::error_exit(anyhow!(err).context("Could not clear screen"))
        }
    }

    pub fn clear_line() {
        if let Err(err) = execute!(io::stdout(), terminal::Clear(ClearType::CurrentLine)) {
            log::error_exit(anyhow!(err).context("Could not clear line"))
        }
    }

    pub fn move_cursor_row(row: usize) -> usize {
        let original = match cursor::position() {
            Ok((_, row)) => row.into(),
            Err(err) => log::error_exit(anyhow!(err).context("Could not get cursor position")),
        };

        if let Err(err) = execute!(io::stdout(), cursor::MoveTo(0, row as u16)) {
            log::error_exit(anyhow!(err).context("Could not move cursor"));
        }

        original
    }

    pub fn columns() -> usize {
        match terminal::size().map(|(columns, _)| columns) {
            Ok(columns) => columns.into(),
            Err(err) => log::error_exit(anyhow!(err).context("Could not retrieve terminal size")),
        }
    }

    pub fn rows() -> usize {
        match terminal::size().map(|(_, rows)| rows) {
            Ok(rows) => rows.into(),
            Err(err) => log::error_exit(anyhow!(err).context("Could not retrieve terminal size")),
        }
    }
}
