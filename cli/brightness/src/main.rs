pub extern crate regex;

use std::process::{ Command };
use std::convert::{ TryFrom };
use std::env::{ vars, args };

use regex::{ Regex };

pub struct Percent ( pub f32 );

impl TryFrom<&str> for Percent {
    type Error = String;
    fn try_from(v: &str) -> Result<Self, Self::Error> {
        let last_char = v.chars().rev().next().unwrap();
        match last_char {
            '%' => {
                let len = v.len();
                let string = v.chars().take(len - 1).collect::<String>();
                match string.parse::<f32>() {
                    Ok(v) => Ok(Percent(if v > 100.0 { 100.0 } else if v < 12.5 { 12.5 } else { v })),
                    Err(e) => Err(format!("{}", e)),
                }
            }
            _ => {
                match v.parse::<f32>() {
                    Ok(v) => Ok(Percent(if v > 100.0 { 100.0 } else if v < 12.5 { 12.5 } else { v })),
                    Err(e) => Err(format!("{}", e)),
                }
            }
        }
    }
}

fn main() {
    let args = args().collect::<Vec<String>>();
    
    let percent = if args.len() >= 2 {
        match Percent::try_from(args[1].as_str()) {
            Ok(v) => v,
            Err(e) => panic!(e),
        }
    } else {
        panic!("Expected a percent value");
    };
    
    let xrandr_query = {
        let try_xrandr_query = Command::new("xrandr")
            .arg("--query")
            .envs(&mut vars())
            .output();
        match try_xrandr_query {
            Ok(xrandr_query_output) => match String::from_utf8(xrandr_query_output.stdout) {
                Ok(xrandr_query) => xrandr_query,
                Err(e) => panic!(e),
            }
            Err(e) => panic!(e),
        }
    };
    
    let regex = Regex::new(r#"(?i)([a-z0-9\-_]+) connected(?: primary)? .*?"#).unwrap();
    for captures in regex.captures_iter(&xrandr_query) {
        Command::new("xrandr")
            .arg("--output")
            .arg(captures.get(1).unwrap().as_str())
            .arg("--brightness")
            .arg(format!("{}", percent.0 / 100.0))
            .output()
            .unwrap(); 
    }

}

