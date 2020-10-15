extern crate chrono;
extern crate num_traits;

use std::thread::{ sleep };
use std::env::{ args };
use std::time::{ Duration };

use chrono::prelude::*;
use num_traits::{ FromPrimitive };

fn update() {
    let now = Local::now();

    let month = {
        let mut to_return = String::new();
        let mut chars = Month::from_u32(now.month()).unwrap().name().chars();
        to_return.push(chars.next().unwrap());
        to_return.push(chars.next().unwrap());
        to_return.push(chars.next().unwrap());
        if to_return.to_lowercase() == "sep" { to_return.push('t'); }
        to_return
    };

    let dom = now.day();

    let dom_suffix = {
        let last_digit = dom % 10;
        let second_to_last_digit = (dom / 10) % 10;
        match second_to_last_digit {
            1 => "th", 
            _ => {
                match last_digit {
                    1 => "st",
                    2 => "nd",
                    3 => "rd",
                    _ => "th",
                }
            }
        }
    };

    println!("{dow}, {month} {dom}{dom_suffix}, {year} -- {hour:02}:{minute:02}{ampm}",
        dow        = now.weekday(),
        month      = month,
        dom        = dom,
        dom_suffix = dom_suffix,
        year       = now.year(),
        hour       = now.hour12().1,
        minute     = now.minute(),
        ampm       = if now.hour12().0 { "PM" } else { "AM" });
}

fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() >= 2 && args[1] == "tail" {
        loop {
            update();
            sleep(Duration::from_millis(1000));
        }
    } else {
        update();
    }
}


