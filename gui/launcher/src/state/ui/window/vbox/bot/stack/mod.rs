use crate::gtk;

use crate::state::data;
use crate::event;
pub mod main;

use gtk::prelude::*;

use event::{ Escalator };
use main::{ main };

pub fn stack(escalator: &Escalator, data: &data::State) -> gtk::Stack {
	let stack = gtk::Stack::new();
	stack.add_named(&main(&escalator, &data), "main");
	stack.set_visible_child_name("main");
	stack
}
