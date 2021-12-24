use crossbeam::{channel::bounded, select};
use rust_pomodoro::Config;
use std::{process, thread};

fn main() {
    let config = Config::new();

    let (is_done_sender, is_done_receiver) = bounded(1);
    let (is_break_sender, is_break_receiver) = bounded(1);
    let (is_start_sender, is_start_receiver) = bounded(1);

    thread::spawn(move || {
        config.pomodoro(is_done_sender, is_break_sender, is_start_sender);
        // rust_pomodoro::pomodoro(config, is_done_sender, is_break_sender, is_start_sender)
    });

    loop {
        select! {
            recv(&is_start_receiver) -> p => {
                if let Ok(_) = p {
                    rust_pomodoro::notify(String::from("Starting"));
                }
            }
            recv(&is_done_receiver) -> p => {
                if let Ok(_) = p {
                    rust_pomodoro::notify(String::from("Finish"));
                }
                process::exit(1);
            }
            recv(&is_break_receiver) -> p => {
                if let Ok(_) = p {
                    rust_pomodoro::notify(String::from("Break"));
                }
            }
        }
    }
}
