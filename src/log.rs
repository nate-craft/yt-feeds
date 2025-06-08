use std::{fs::File, io::Write, process};

use chrono::Local;
use crossterm::style::Stylize;

use crate::{cache::data_directory, clear_screen};

pub fn err_and_exit(message: impl ToString) -> ! {
    clear_screen();
    eprintln!("{}", message.to_string().red().italic());
    err(message);
    process::exit(0);
}

pub fn err(message: impl ToString) {
    let result = data_directory()
        .map_err(|_| "Data directory could not be founded!")
        .map(|path| path.join("logs.txt"))
        .and_then(|path| {
            File::options()
                .append(true)
                .create(true)
                .open(path)
                .map_err(|_| "Log file could not be created!")
        })
        .and_then(|mut file| {
            let formatted = format!(
                "[{}]: {}\n",
                Local::now().format("%d/%m/%Y-%H:%M"),
                message.to_string()
            );
            file.write_all(formatted.as_bytes())
                .map_err(|_| "Log file could not be modified!")
        });

    if let Err(err) = result {
        eprintln!(
            "Catastrophic error: Could not load log file. Reason\n {}",
            err
        );
        process::exit(1);
    }
}
