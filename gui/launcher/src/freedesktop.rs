use crate::log;

use std::path::{ Path, PathBuf };
use std::fs::{ read_to_string };
use std::collections::{ HashMap };
use std::fmt;

use log::{ error, warn };

#[derive(Clone)]
pub struct DesktopEntry {
	pub path: PathBuf,
	pub icon: String,
	pub name: String,
	pub description: String,
	pub keywords: Vec<String>,
	pub exec: String,
	pub terminal: bool,
	pub no_display: bool,
	pub actions: Actions,
	pub categories: Categories,
}

impl DesktopEntry {
	pub fn new<P: AsRef<Path>>(p: P) -> Self where P: fmt::Debug {
		let mut to_return = DesktopEntry {
			path: p.as_ref().to_path_buf(),
			icon: String::new(),
			name: String::new(),
			description: String::new(),
			keywords: Vec::new(),
			exec: String::new(),
			terminal: false,
			no_display: false,
			actions: Actions::new(),
			categories: Categories::new(),
		};

		let file = match read_to_string(p.as_ref()) {
			Ok(v) => v,
			Err(e) => { error!("Failed to read {:?}. Details:{}", &p, e); return to_return; }
		};

		let lines = file.split("\n").collect::<Vec<&str>>();

		enum Mode {
			Default,
			DesktopEntry,
			DesktopAction(String),
            Skip,
		}

		let mut mode = Mode::Default;

		let mut actions: HashMap<String, Action> = HashMap::new();

		for line in lines.iter() {
			match mode {
				Mode::Default => {
					if line.starts_with("[") {
						if line.starts_with("[Desktop Entry]") {
							mode = Mode::DesktopEntry;
						} else if line.starts_with("[Desktop Action ") {
							let action = line.replace("[Desktop Action ", "").replace("]", "");
							actions.insert(action.clone(), Action {
								name: String::new(),
								exec: String::new(),
							});
							mode = Mode::DesktopAction(action);
						} else {
							warn!("Unexpected [] value in {:?}: {:?}", p, line);
                            mode = Mode::Skip;
						}
					}
				}
				Mode::DesktopEntry => {
					if line.contains("=") {
						let split = line.splitn(2, "=").collect::<Vec<&str>>();
						match split[0] {
							"Name" => to_return.name = String::from(split[1]),
							"Comment" => to_return.description = String::from(split[1]),
							"Icon" => to_return.icon = String::from(split[1]),
							"Exec" => to_return.exec = String::from(split[1]),
							"Terminal" => to_return.terminal = split[1].to_lowercase().starts_with("t"),
							"NoDisplay" => to_return.no_display = split[1].to_lowercase().starts_with("t"),
							"GenericName" => {
								if to_return.description == "" {
									to_return.description = String::from(split[1]);
								}
							}
							"Keywords" => {
								let mut keywords_vec_str =
									split[1].split(";").collect::<Vec<&str>>();
								keywords_vec_str.pop();
								let mut keywords_vec_string = Vec::new();
								for keyword in keywords_vec_str.iter() {
									keywords_vec_string.push(String::from(*keyword));
								}
								to_return.keywords = keywords_vec_string;
							}
							"Categories" => {
								let mut categories_vec_str =
									split[1].split(";").collect::<Vec<&str>>();
								categories_vec_str.pop();
								let mut categories_vec = Vec::new();
								for category in categories_vec_str.iter() {
									categories_vec.push(Category::from(category));
								}
								to_return.categories = categories_vec;
							}
							_ => (),
						}
					} else if line.starts_with("[") {
						if line.starts_with("[Desktop Entry]") {
							mode = Mode::DesktopEntry;
						} else if line.starts_with("[Desktop Action ") {
							let action = line.replace("[Desktop Action ", "").replace("]", "");
							actions.insert(action.clone(), Action {
								name: String::new(),
								exec: String::new(),
							});
							mode = Mode::DesktopAction(action);
						} else {
							warn!("Unexpected [] value in {:?}: {:?}", p, line);
                            mode = Mode::Skip;
						}
					}
				}
				Mode::DesktopAction(ref action) => {
					if line.contains("=") {
						let split = line.splitn(2, "=").collect::<Vec<&str>>();
						match split[0] {
							"Name" => {
								let mut old_action = actions.get(action).unwrap().clone();
								old_action.name = line.replace("Name=", "");
								actions.insert(action.clone(), old_action);
							}
							"Exec" => {
								let mut old_action = actions.get(action).unwrap().clone();
								old_action.exec = line.replace("Exec=", "");
								actions.insert(action.clone(), old_action);
							}
							_ => (),
						}
					} else if line.starts_with("[") {
						if line.starts_with("[Desktop Entry]") {
							mode = Mode::DesktopEntry;
						} else if line.starts_with("[Desktop Action ") {
							let action = line.replace("[Desktop Action ", "").replace("]", "");
							actions.insert(action.clone(), Action {
								name: String::new(),
								exec: String::new(),
							});
							mode = Mode::DesktopAction(action);
						} else {
							warn!("Unexpected [] value in {:?}: {:?}", p, line);
                            mode = Mode::Skip;
						}
					}
				}
                Mode::Skip => {
					if line.starts_with("[") {
						if line.starts_with("[Desktop Entry]") {
							mode = Mode::DesktopEntry;
						} else if line.starts_with("[Desktop Action ") {
							let action = line.replace("[Desktop Action ", "").replace("]", "");
							actions.insert(action.clone(), Action {
								name: String::new(),
								exec: String::new(),
							});
							mode = Mode::DesktopAction(action);
						} else {
							warn!("Unexpected [] value in {:?}: {:?}", p, line);
                            mode = Mode::Skip;
						}
					}
                }
			}
		}

		for key in actions.keys() {
			to_return.actions.push(actions.get(key).unwrap().clone());
		}

		to_return
	}
}

impl fmt::Display for DesktopEntry {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(f, "DesktopEntry {{")?;
		writeln!(f, "\tname: {:?},", self.name)?;
		writeln!(f, "\tdescription: {:?},", self.description)?;
		writeln!(f, "\t/* fields omitted */")?;
		write!(f, "}}")
	}
}

impl fmt::Debug for DesktopEntry {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "DesktopEntry {{ name: {:?}, /* fields omitted */ }}", self.name)
	}
}

pub type Actions = Vec<Action>;
#[derive(Clone, Debug)]
pub struct Action {
	pub name: String,
	pub exec: String,
}

pub type Categories = Vec<Category>;
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Category {
	Development,
	Network,
	Settings,
	Graphics,
	Utility,
	Game,
	Audio,
	Video,
	AudioVideo,
	System,
	Other(String),
}

impl Category {
	pub fn from(v: &str) -> Category {
		match v {
			"Development" => Category::Development,
			"Network" => Category::Network,
			"Settings" => Category::Settings,
			"Game" => Category::Game,
			"Graphics" => Category::Graphics,
			"Utility" => Category::Utility,
			"Audio" => Category::Audio,
			"Video" => Category::Video,
			"AudioVideo" => Category::AudioVideo,
			"System" => Category::System,
			other => Category::Other(String::from(other)),
		}
	}
}

