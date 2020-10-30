use crate::freedesktop;

use std::fs::{ read_to_string };
use std::path::{ Path, };
use std::env::{ var };
use std::cmp::{ Ordering };

use freedesktop::{ DesktopEntry };

/// An interface for `~/.cache/rofi-3.druncache`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State ( Vec<(usize, String)> );

impl State {
    pub fn new() -> Self {
		let mut to_return = Vec::new();

        let path = Path::new("/home")
            .join(var("USER").unwrap_or(String::new()))
            .join(".cache/rofi3.druncache");

        let file = match read_to_string(&path) {
            Ok(file) => file,
            Err(err) => panic!(format!("\n\nFailed to read {path:?}.\n\nDetails:\n\n{err}\n\n",
                path = path,
                err = err)),
        };

        let lines = file.split("\n").collect::<Vec<&str>>();

        for i in 0..lines.len() - 2 {
            let line = lines[i];
            let split = line.splitn(2, " ").collect::<Vec<&str>>();
            let n = match split[0].parse::<usize>() {
                Ok(n) => n,
                Err(e) => panic!(format!("\n\nFailed to parse {v:?} as usize.\n\nDetails:\n\n{err}\n\n",
                    v = split[0],
                    err = e)),
            };
            let s = String::from(split[1]);
            to_return.push((n, s));
        }

        to_return.sort_by(|a, b| {
            if a.0 > b.0 {
                Ordering::Less
            } else if a.0 < b.0 {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
		});
		
        State(to_return)
    }
	
    pub fn desktop_entries(&self) -> Vec<DesktopEntry> {
        let mut to_return = Vec::new();
        for app in self.0.iter() {
            let path = Path::new("/usr/share/applications")
                .join(app.1.clone());
            if path.exists() {
                to_return.push(DesktopEntry::new(&path));
            } else {
                let path = Path::new("/home")
                    .join(var("USER").unwrap_or(String::new()))
                    .join(".local/share/applications")
                    .join(app.1.clone());
                if path.exists() {
                    to_return.push(DesktopEntry::new(&path));
                }
            }
        }
        to_return
    }
}
