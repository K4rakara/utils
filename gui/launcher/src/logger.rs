use crate::chrono;
use crate::log;

use std::time::{ Duration };

use chrono::{ Local, Timelike };
use log::{ Record, Level, Log, Metadata };

pub static LOGGER: Logger = Logger;

fn now() -> String {
    let now = Local::now();
    format!("{hour:02}:{minute:02}:{second:02}:{millisecond:04}{ampm}",
        hour = now.hour12().1,
        minute = now.minute(),
        second = now.second(),
        millisecond = Duration::from_nanos(now.nanosecond().into()).as_millis(),
        ampm = if now.hour12().0 { "PM" } else { "AM" })
}

pub struct Logger;

impl Log for Logger {
    fn enabled(&self, _: &Metadata) -> bool { true }
    fn log(&self, r: &Record) {
        match r.level() {
            Level::Info => println!("[\u{001b}[38;5;4m   INFO   \u{001b}[0m][  {time: ^15}  ]: {args}",
                time = now(),
                args = r.args()),
            Level::Warn => println!("[\u{001b}[38;5;3m   WARN   \u{001b}[0m][  {time: ^15}  ]: {args}",
                time = now(),
                args = r.args()),
            Level::Error => println!("[\u{001b}[38;5;1m  ERROR   \u{001b}[0m][  {time: ^15}  ]: {args}",
                time = now(),
                args = r.args()),
            Level::Trace => println!("[\u{001b}[38;5;5m  TRACE   \u{001b}[0m][  {time: ^15}  ]: {args}",
                time = now(),
                args = r.args()),
            _ => (),
        }
    }
    fn flush(&self) {}
}

