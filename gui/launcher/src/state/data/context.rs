use std::env::{ var };
use std::fs::{ read_to_string };

#[derive(Clone, Debug)]
pub struct State {
	pub user: String,
	pub host: String,
}

impl State {
	pub fn new() -> Self {
		State {
			user: var("USER").unwrap_or(String::from("user")),
			host: match read_to_string("/etc/hostname") {
				Ok(v) => v.replace("\n", ""),
				Err(e) => panic!(format!("\n\nFailed to read /etc/hostname.\n\nDetails:\n\n{}\n\n", e)),
			}
		}
	}
}
