use std::{thread, time::Duration};

use crate::clear_screen;

// run task T with loading action (usually printing 1-3 lines) in print_fn
pub fn while_loading<T, P, R>(task: T, print_fn: P) -> R
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
