use crate::gtk;

use crate::state::data;
use crate::event;
pub mod actions;
pub mod stack;

use gtk::prelude::*;

use event::{ Escalator };
use actions::{ actions };
use stack::{ stack };

#[derive(Clone, Debug)]
pub struct State {
	pub stack: gtk::Stack,
	pub inner: gtk::Box,
}

impl State {
	pub fn new(escalator: &Escalator, data: &data::State) -> Self {
		let bot = gtk::Box::new(gtk::Orientation::Vertical, 0);
		bot.set_widget_name("bot");
		
		let stack = stack(&escalator, &data);
		bot.add(&stack);
		
		let actions = actions(&escalator);
		bot.add(&actions);
		
		State { stack, inner: bot }
	}
}

