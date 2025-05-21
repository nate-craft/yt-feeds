use crate::clear_screen;
use crate::view::Error;
use colored::Colorize;
use crossterm::event::Event;
use crossterm::{cursor, event, execute, terminal};
use std::io::Write;
use std::process::Child;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{io, thread};

pub fn process_while_loading<P>(task: io::Result<Child>, print_fn: P) -> Result<(), Error>
where
    P: Fn() + Send + 'static,
{
    let (tx, rx) = std::sync::mpsc::channel::<bool>();
    let (tx_end, rx_end) = std::sync::mpsc::channel::<bool>();
    let (tx_in, rx_in) = std::sync::mpsc::channel::<bool>();

    clear_screen();
    execute!(io::stdout(), cursor::MoveTo(0, 1)).expect("Failed to move cursor");

    // output thread
    thread::spawn(move || {
        let mut step = 0;
        let steps = ["⢿", "⣻", "⣽", "⣾", "⣷", "⣯", "⣟", "⡿"];

        // Repeated to get the position for later
        execute!(io::stdout(), cursor::MoveTo(0, 1)).unwrap();
        print_fn();
        println!("  {}", steps[step]);
        print!(
            "\n{}\n\n▶ ",
            "Options: [(d)etach, (c)ancel]".green().italic()
        );
        io::stdout().flush().unwrap();

        while let Err(_) = rx.recv_timeout(Duration::from_millis(150)) {
            execute!(io::stdout(), cursor::SavePosition).unwrap();
            execute!(io::stdout(), cursor::MoveTo(0, 1)).unwrap();
            print_fn();
            println!("  {}", steps[step]);
            println!(
                "\n{}\n\n{}",
                "Options: [(d)etach, (c)ancel]".green().italic(),
                "▶"
            );

            execute!(io::stdout(), cursor::RestorePosition).unwrap();
            step = (step + 1) % steps.len();

            thread::sleep(std::time::Duration::from_millis(300));
        }
    });

    match task {
        Ok(command) => {
            let command = Arc::new(Mutex::new(command));
            let command_finished = command.clone();

            let tx_finished = tx.clone();
            let tx_end_finished = tx_end.clone();

            // task monitoring thread
            thread::spawn(move || loop {
                if let Ok(result) = command_finished.lock().unwrap().try_wait() {
                    if let Some(status) = result {
                        let _ = tx_finished.send(true);
                        let _ = tx_in.send(true);
                        if status.success() {
                            clear_screen();
                            // Needed to finish input thread stdin readline
                            print!(
                                "\n{}{}",
                                "External process finished. Press enter to continue...\n\n"
                                    .green()
                                    .italic(),
                                "▶ "
                            );
                            io::stdout().flush().unwrap();
                        }
                        return;
                    }
                } else {
                    let _ = tx_finished.send(true);
                    let _ = tx_end_finished.send(true);
                    let _ = tx_in.send(true);
                    return;
                }
                if event::poll(std::time::Duration::from_millis(1)).unwrap() {
                    match event::read().unwrap() {
                        Event::Resize(_, _) => {
                            execute!(io::stdout(), cursor::SavePosition).unwrap();
                            clear_screen();
                            execute!(io::stdout(), cursor::RestorePosition).unwrap();
                        }
                        _ => {}
                    }
                }

                thread::sleep(std::time::Duration::from_millis(100));
            });

            // input thread
            thread::spawn(move || {
                while let Err(_) = rx_in.try_recv() {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let key = input.trim().to_lowercase();
                    if key.eq("c") {
                        let _ = tx.send(true);
                        let _ = command.lock().unwrap().kill();
                        let _ = tx_end.send(true);
                        return;
                    } else if key.eq("d") {
                        let _ = tx.send(true);

                        let _ = tx_end.send(true);
                        return;
                    }
                    execute!(io::stdout(), cursor::MoveUp(1), cursor::MoveRight(2)).unwrap();
                    execute!(
                        io::stdout(),
                        terminal::Clear(terminal::ClearType::FromCursorDown)
                    )
                    .unwrap();
                }
            });
        }
        Err(e) => return Err(Error::CommandFailed(e.to_string())),
    }

    while let Err(_) = rx_end.recv() {
        return Ok(());
    }
    return Ok(());
}

pub fn run_while_loading<T, P, R>(task: T, print_fn: P) -> R
where
    T: FnOnce() -> R + Send,
    P: Fn() + Send + 'static,
    R: Send,
{
    let (tx, rx) = std::sync::mpsc::channel::<bool>();

    thread::spawn(move || {
        let mut step = 0;
        let steps = ["⢿", "⣻", "⣽", "⣾", "⣷", "⣯", "⣟", "⡿"];

        while let Err(_) = rx.recv_timeout(Duration::from_millis(150)) {
            clear_screen();
            print_fn();
            println!("  {}", steps[step]);
            step = (step + 1) % steps.len();
            step = step + 1;
            if step > steps.len() - 1 {
                step = 0;
            }

            thread::sleep(std::time::Duration::from_millis(300));
        }
    });

    let result = task();
    tx.send(true).unwrap();

    result
}
