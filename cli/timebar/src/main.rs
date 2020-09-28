extern crate chrono;
extern crate num_traits;

use chrono::prelude::*;
use num_traits::{ FromPrimitive };

fn main() {
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
        let dom_string = format!("{}", dom);
        let (last_char, second_to_last_char) = {
            let mut chars = dom_string.chars().rev();
            (chars.next().unwrap(), chars.next().unwrap_or('0'))
        };
        match second_to_last_char {
            '1' => "th", 
            ___ => {
                match last_char {
                    '1' => "st",
                    '2' => "nd",
                    '3' => "rd",
                    ___ => "th",
                }
            }
        }
    };

    print!("{dow}, {month} {dom}{dom_suffix}, {year} -- {hour:02}:{minute:02}{ampm}",
        dow        = now.weekday(),
        month      = month,
        dom        = dom,
        dom_suffix = dom_suffix,
        year       = now.year(),
        hour       = now.hour12().1,
        minute     = now.minute(),
        ampm       = if now.hour12().0 { "PM" } else { "AM" });
}

