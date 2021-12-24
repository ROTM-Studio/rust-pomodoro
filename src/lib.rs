use clap::Parser;
use crossbeam::channel::Sender;
use std::{
    io::{stdout, Write},
    process::Command,
    thread,
    time::Duration,
};

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Pomodoro {
    #[clap(long, parse(try_from_str))]
    pub focus_time: u32,
    #[clap(long, parse(try_from_str))]
    pub break_time: u32,
    #[clap(long, parse(try_from_str))]
    pub session: u32,
}

impl Pomodoro {
    pub fn new() -> Pomodoro {
        Pomodoro::parse()
    }

    pub fn run(&self, is_done: Sender<bool>, is_break: Sender<bool>, is_start: Sender<u32>) {
        let focus_in_second = self.focus_time * 60;
        let break_in_second = self.break_time * 60;

        let mut stdout = stdout();

        for i in 0..self.session {
            is_start.send(i + 1).unwrap();

            print!("\x1b[41m");
            print!("\x1b[37m");

            for i in (0..focus_in_second).rev() {
                let minute_remaining = i / 60;
                let second_remaining = i % 60;
                print!("\r{}:{} - Focus ", minute_remaining, second_remaining);
                stdout.flush().unwrap();
                thread::sleep(Duration::from_secs(1));
            }

            if i == self.session - 1 {
                stdout.flush().unwrap();
                print!("\r");
                break;
            }

            is_break.send(true).unwrap();

            print!("\x1b[42m");
            print!("\x1b[30m");

            for i in (0..break_in_second).rev() {
                let minute_remaining = i / 60;
                let second_remaining = i % 60;
                print!("\r{}:{} - Break ", minute_remaining, second_remaining);
                stdout.flush().unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        }

        is_done.send(true).unwrap();
    }
}

pub fn notify(message: String) {
    Command::new("notify-send")
        .arg("-t")
        .arg("3000")
        .arg(message)
        .spawn()
        .expect("failed to execute process");
}
