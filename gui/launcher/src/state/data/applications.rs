use super::rofi;
use crate::freedesktop;

use std::env::{ var };
use std::fs::{ read_to_string, read_dir };
use std::path::{ Path };
use std::collections::{ HashMap };
use std::cmp::{ Ordering };

use freedesktop::{ Category, DesktopEntry };

#[derive(Clone, Debug)]
pub struct State {
	pub all: Vec<DesktopEntry>,
	pub favorites: Vec<DesktopEntry>,
	pub recent: Vec<DesktopEntry>,
	pub by_category: ByCategory,
}

impl State {
	pub fn new(rofi: &rofi::State) -> Self {
		let favorites = {
			let mut to_return = Vec::new();
			
			let path = Path::new("/home")
				.join(var("USER").unwrap_or(String::from("")))
				.join(".config/launcher/favorites");

			let file = match read_to_string(path) {
				Ok(file) => file,
				Err(e) => panic!(format!("\n\nFailed to read /home/{user}/.config/launcher/favorites\n\nDetails:\n\n{err}",
					user = var("USER").unwrap_or(String::from("")),
					err = e)),
			};

			let lines = file.split("\n").collect::<Vec<&str>>();

			for line in lines.iter() {
				if *line != "" {
					to_return.push(freedesktop::DesktopEntry::new(*line));
				}
			}

			to_return
		};

		let mut all = Vec::new();
		let mut audiovideo = Vec::new();
		let mut lost_and_found = Vec::new();
		let recent = rofi.desktop_entries()[0..6].to_vec();

		let mut catmap: HashMap<Category, Vec<DesktopEntry>> = [
			( Category::Development, Vec::new() ),
			( Category::Game,        Vec::new() ),
			( Category::Graphics,    Vec::new() ),
			( Category::Network,     Vec::new() ),
			( Category::Settings,    Vec::new() ),
			( Category::System,      Vec::new() ),
			( Category::Utility,     Vec::new() ),
		].iter().cloned().collect();

		let mut load_desktop_entries_from = |from: Box<dyn AsRef<Path>>| {
			if let Ok(entries) = read_dir(from.as_ref()) {
				for entry in entries {
					if let Ok(entry) = entry {
						let desktop_entry = DesktopEntry::new(entry.path());
						if !desktop_entry.no_display
						&& desktop_entry.name != ""
						&& desktop_entry.description != ""
						&& desktop_entry.icon != "" {
							let mut in_category = false;
							let mut in_audiovideo = false;
							for category in desktop_entry.categories.iter() {
								match category {
										Category::Audio
									|Category::Video
									|Category::AudioVideo => {
										if !in_audiovideo {
											audiovideo.push(desktop_entry.clone());
											in_category = true;
											in_audiovideo = true;
										}
									}
									Category::Other(_) => (),
									category => {
										catmap.get_mut(category).unwrap().push(desktop_entry.clone());
										in_category = true;
									},
								}
							}
							if !in_category { lost_and_found.push(desktop_entry.clone()); }
							all.push(desktop_entry.clone());
						}
					}
				}
			}
		};

		// Load desktop entries.
		load_desktop_entries_from(Box::new("/usr/share/applications"));
		load_desktop_entries_from(
			Box::new(
				Path::new("/home/")
					.join(var("USER").unwrap_or(String::new()))
					.join(".local/share/applications")));

		// Sort categories alphabetically.
		{
			let sort = |a: &DesktopEntry, b: &DesktopEntry| -> Ordering {
				let a_name = a.name.to_lowercase();
				let b_name = b.name.to_lowercase();
				
				if a_name == b_name { return Ordering::Equal; }

				let mut a_name_chars = a_name.chars();
				let mut b_name_chars = b_name.chars();

				loop {
					let a_this_char = a_name_chars.next();
					let b_this_char = b_name_chars.next();
					
					if a_this_char.is_some() {
						if !b_this_char.is_some() {
							break Ordering::Greater;
						} else {
							let a_this_char = a_this_char.unwrap() as u8;
							let b_this_char = b_this_char.unwrap() as u8;

							if a_this_char > b_this_char {
								break Ordering::Greater;
							} else if a_this_char < b_this_char {
								break Ordering::Less;
							}
						}
					} else {
						if b_this_char.is_some() {
							break Ordering::Less;
						} else {
							break Ordering::Equal;
						}
					}
				}
			};
			all.sort_by(sort);
			audiovideo.sort_by(sort);
			lost_and_found.sort_by(sort);
			catmap.get_mut(&Category::Development).unwrap().sort_by(sort);
			catmap.get_mut(&Category::Game       ).unwrap().sort_by(sort);
			catmap.get_mut(&Category::Graphics   ).unwrap().sort_by(sort);
			catmap.get_mut(&Category::Network    ).unwrap().sort_by(sort);
			catmap.get_mut(&Category::Settings   ).unwrap().sort_by(sort);
			catmap.get_mut(&Category::System     ).unwrap().sort_by(sort);
			catmap.get_mut(&Category::Utility    ).unwrap().sort_by(sort);
		}

		State {
			all: all,
			favorites: favorites,
			recent: recent,
			by_category: ByCategory {
				development:    catmap.get(&Category::Development).unwrap().clone(),
				game:           catmap.get(&Category::Game       ).unwrap().clone(),
				graphics:       catmap.get(&Category::Graphics   ).unwrap().clone(),
				network:        catmap.get(&Category::Network    ).unwrap().clone(),
				settings:       catmap.get(&Category::Settings   ).unwrap().clone(),
				system:         catmap.get(&Category::System     ).unwrap().clone(),
				utility:        catmap.get(&Category::Utility    ).unwrap().clone(),
				audiovideo:     audiovideo,
				lost_and_found: lost_and_found,
			}
		}
	}
}

#[derive(Clone, Debug)]
pub struct ByCategory {
	pub development: Vec<DesktopEntry>,
	pub network: Vec<DesktopEntry>,
	pub settings: Vec<DesktopEntry>,
	pub graphics: Vec<DesktopEntry>,
	pub utility: Vec<DesktopEntry>,
	pub game: Vec<DesktopEntry>,
	pub audiovideo: Vec<DesktopEntry>,
	pub system: Vec<DesktopEntry>,
	pub lost_and_found: Vec<DesktopEntry>,
}