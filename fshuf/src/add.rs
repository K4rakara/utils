use crate::prefix;

use std::fs;
use std::path::{ Path };
use std::env::{ current_dir };
use std::process::{ exit };

use prefix::Prefix;

pub(crate) fn add(prefix: Option<String>) {
	let pwd = match current_dir() {
		Ok(v) => v,
		Err(_) => {
			println!("\u{001b}[38;5;1mError:\u{001b}[0m Failed to get current directory.");
			exit(4);
		}
	};
	let prefix_string = match prefix {
		Some(v) => v,
		None => String::from("dddd"),
	};
	let prefix = Prefix::new_from(&prefix_string);
	match fs::read_dir(&pwd) {
		Ok(entries) => {
			for try_entry in entries {
				match try_entry {
					Ok(entry) => {
						match entry.path().file_name() {
							Some(file_name) => {
								let new_file_name = format!("{prefix}-{file_name}",
									prefix = prefix.generate(),
									file_name = file_name.to_string_lossy());
								match fs::rename(file_name, Path::new(&new_file_name)) {
									Ok(_) => (),
									Err(e) => {
										println!("\u{001b}[38;5;3mWarning:\u{001b}[0m Failed to rename \"{from}\" to \"{to}\". Details:\n{err}",
											from = file_name.to_string_lossy(),
											to = new_file_name,
											err = e);
									}
								}
							}
							None => (), // This _shouldn't_ fail, except when it lists out "..", which can be ignored.
						}
					}
					Err(e) => {
						println!("\u{001b}[38;5;3mWarning:\u{001b}[0m The following I/O error occured:\n{err}",
							err = e);
						exit(6);
					}
				} 
			}
		}
		Err(e) => {
			println!("\u{001b}[38;5;3mError:\u{001b}[0m An uncatchable I/O error occured. Details:\n{err}",
				err = e);
			exit(5);
		}
	}
	match fs::write(pwd.join(".fshuf"), &prefix_string) {
		Ok(_) => (),
		Err(_) => (),
	}
}
