use crate::clear_screen;
use crate::view::Error;
use crossterm::event;
use crossterm::event::KeyCode;
use crossterm::style::Stylize;
use std::process::Child;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{io, thread};

struct Flags<T> {
    running: AtomicBool,
    detached: AtomicBool,
    result: Arc<Mutex<Option<T>>>,
}

impl<T> Flags<T> {
    fn atomic() -> Arc<Flags<T>> {
        Arc::new(Flags {
            running: AtomicBool::new(true),
            detached: AtomicBool::new(false),
            result: Arc::new(Mutex::new(None)),
        })
    }

    fn deny_use(&self) -> bool {
        !self.running() || self.detached()
    }

    fn running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    fn detached(&self) -> bool {
        self.detached.load(Ordering::Relaxed)
    }

    fn set_running(self: &Arc<Self>, setting: bool) {
        self.running.store(setting, Ordering::Relaxed);
    }

    fn set_detached(self: &Arc<Self>, setting: bool) {
        self.detached.store(setting, Ordering::Relaxed);
    }

    fn set_result(self: &Arc<Self>, result: Option<T>) {
        if let Ok(mut res) = self.result.try_lock() {
            *res = result;
        }
    }
}

pub fn cmd_while_loading<P>(task: io::Result<Child>, print_fn: P) -> Result<(), Error>
where
    P: Fn() + Send + 'static,
{
    cmd_while_loading_with_background::<_, _, fn() -> Option<()>>(task, print_fn, None).map(|_| ())
}

pub fn cmd_while_loading_with_background<P, T, R>(
    task: io::Result<Child>,
    print_fn: P,
    background_fn: Option<Box<R>>,
) -> Result<Option<T>, Error>
where
    P: Fn() + Send + 'static,
    T: Send + Sync + 'static + Clone + Copy + core::fmt::Debug,
    R: Send + Sync + 'static + Fn() -> Option<T>,
{
    let steps = ["⢿", "⣻", "⣽", "⣾", "⣷", "⣯", "⣟", "⡿"];
    let mut step = 0;
    let flags: Arc<Flags<T>> = Flags::atomic();
    crossterm::terminal::enable_raw_mode().unwrap();

    // Background Return Task - stops on detach/cancellation
    if let Some(background) = background_fn {
        thread::spawn({
            let flags = Arc::clone(&flags);
            move || loop {
                if flags.deny_use() {
                    return;
                }

                let result = background();
                if result.is_some() {
                    flags.set_result(result);
                }

                thread::sleep(std::time::Duration::from_secs(1));
            }
        });
    }

    // Output - stops on detach/cancellation
    thread::spawn({
        let flags = Arc::clone(&flags);
        move || loop {
            if flags.deny_use() {
                return;
            }

            clear_screen();
            print_fn();
            print!("  {}\r\n", steps[step]);
            print!(
                "\r\n{}\r\n",
                "Options: [(d)etach, (c)ancel]".green().italic()
            );

            step = (step + 1) % steps.len();
            thread::sleep(std::time::Duration::from_millis(450));
        }
    });

    // Input - stop by detach/cancellation
    thread::spawn({
        let flags = Arc::clone(&flags);
        move || loop {
            if flags.deny_use() {
                return;
            }

            if event::poll(Duration::from_millis(500)).unwrap() {
                if let event::Event::Key(key_event) = event::read().unwrap() {
                    if key_event.code == KeyCode::Char('c') {
                        flags.set_running(false);
                    } else if key_event.code == KeyCode::Char('d') {
                        flags.set_detached(true);
                    }
                }
            }
        }
    });

    // Main Task - stop by detach/cancellation
    match task {
        Ok(command) => {
            let command = Arc::new(Mutex::new(command));
            thread::spawn({
                let flags = Arc::clone(&flags);
                move || loop {
                    if flags.detached() {
                        return;
                    }

                    // other thread signals stop, kill command
                    if !flags.running() {
                        let _ = command.lock().unwrap().kill();
                        return;
                    }

                    // stop all if finished
                    if let Ok(Some(_)) = command.lock().unwrap().try_wait() {
                        flags.set_running(false);
                        return;
                    }
                }
            });
        }
        Err(e) => {
            crossterm::terminal::disable_raw_mode().unwrap();
            return Err(Error::CommandFailed(e.to_string()));
        }
    }

    // Await till quit, failed, or detached
    loop {
        if flags.deny_use() {
            crossterm::terminal::disable_raw_mode().unwrap();
            return flags
                .result
                .try_lock()
                .map_err(|e| Error::InternalError(e.to_string()))
                .map(|guard| *guard);
        }
    }
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

        while rx.recv_timeout(Duration::from_millis(150)).is_err() {
            clear_screen();
            print_fn();
            println!("  {}", steps[step]);
            step = (step + 1) % steps.len();
            step += 1;
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
