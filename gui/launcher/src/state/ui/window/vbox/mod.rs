use crate::gtk;

use crate::state::data;
use crate::event;
pub mod top;
pub mod bot;

use gtk::prelude::*;

use event::{ Escalator };

#[derive(Clone, Debug)]
pub struct State {
	pub top: top::State,
	pub bot: bot::State,
}

impl State {
	pub fn new(data: &data::State, win: &gtk::Window, escalator: &Escalator) -> Self {
		let vbox = gtk::Box::new(gtk::Orientation::Vertical, 8);
		vbox.set_widget_name("root");
		let top = top::State::new(&data, &vbox, &escalator);
		
		let bot = bot::State::new(&escalator, &data);
		vbox.add(&bot.inner);

		win.add(&vbox);
		State {
			top,
			bot,
		}
	}
}

