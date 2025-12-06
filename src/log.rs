use std::{fs::File, io::Write, process};

use anyhow::anyhow;
use chrono::Local;
use crossterm::style::Stylize;

use crate::{
    display::screen::Screen,
    input::Input,
    storage::directory::{Directory, Dirs},
};

pub fn error_exit(error: anyhow::Error) -> ! {
    error_exit_msg(&error.to_string())
}

pub fn error_exit_msg(message: &str) -> ! {
    Screen::clear();
    error_log(message);
    let _ = Input::typing_enable();
    eprintln!("{}", message.red().italic());
    process::exit(1);
}

pub fn error_log(message: &str) {
    let result = Directory::try_from(Dirs::Data)
        .map_err(|e| anyhow!(e).context("yt-feeds data directory could not be found!"))
        .and_then(|path| path.file("logs.txt"))
        .and_then(|path| {
            File::options()
                .append(true)
                .create(true)
                .open(path)
                .map_err(|e| anyhow!(e).context("Erorr logging file could not be created!"))
        })
        .and_then(|mut file| {
            let formatted = format!(
                "[{}]: {}\n",
                Local::now().format("%d/%m/%Y-%H:%M"),
                message.to_string()
            );
            file.write_all(formatted.as_bytes())
                .map_err(|e| anyhow!(e).context("Log file could not be modified!"))
        });

    if let Err(err) = result {
        let _ = Input::typing_enable();
        eprintln!(
            "  Catstrophic error: \n{}\n  Original error: {}",
            err.to_string().red().italic(),
            message.red().italic(),
        );
        process::exit(1);
    }
}
