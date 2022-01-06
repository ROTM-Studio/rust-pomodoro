use clap::Parser;
use crossbeam::channel::Sender;
use std::{
    io::{stdout, Write},
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
    pub sesion: u32,
}

impl Pomodoro {
    pub fn new() -> Pomodoro {
        Pomodoro::parse()
    }

    pub fn run(&self, is_done: Sender<bool>, is_break: Sender<bool>, is_start: Sender<u32>) {
        let focus_in_second = self.focus_time * 60;
        let break_in_second = self.break_time * 60;

        let calculate_minute_and_second_remaining = |i: u32| -> (u32, u32) { (i / 60, i % 60) };

        let mut stdout = stdout();

        for i in 0..self.sesion {
            is_start.send(i + 1).unwrap();

            print!("\x1b[41m");
            print!("\x1b[37m");

            for i in (0..focus_in_second).rev() {
                let (minute, second) = calculate_minute_and_second_remaining(i);
                print!("\r{}:{} - Focus ", minute, second);
                stdout.flush().unwrap();
                thread::sleep(Duration::from_secs(1));
            }

            if i == self.sesion - 1 {
                stdout.flush().unwrap();
                print!("\r");
                break;
            }

            is_break.send(true).unwrap();

            print!("\x1b[42m");
            print!("\x1b[30m");

            for i in (0..break_in_second).rev() {
                let (minute, second) = calculate_minute_and_second_remaining(i);
                print!("\r{}:{} - Break ", minute, second);
                stdout.flush().unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        }

        is_done.send(true).unwrap();
    }
}
