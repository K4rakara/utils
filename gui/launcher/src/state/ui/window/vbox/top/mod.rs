use crate::gtk;

use crate::state::data;
use crate::event;
pub mod ctx;

use gtk::prelude::*;

use event::{ Escalator };

#[derive(Clone, Debug)]
pub struct State {
	pub ctx: ctx::State,
	pub inner: gtk::Box,
	pub escalator: Escalator,
}

impl State {
	pub fn new(data: &data::State, parent: &gtk::Box, escalator: &Escalator) -> Self {
		let top = gtk::Box::new(gtk::Orientation::Horizontal, 0);
		let ctx = ctx::State::new(&data, &top, &escalator);
		top.set_widget_name("top");
		parent.add(&top);
		State {
			ctx,
			inner: top,
			escalator: escalator.clone(),
		}
	}
}
