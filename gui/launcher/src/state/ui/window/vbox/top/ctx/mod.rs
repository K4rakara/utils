use crate::gtk;
use crate::cairo;

use crate::state::data;
use crate::event;

use std::fs::{ File };
use std::env::{ var };
use std::path::{ Path };

use gtk::prelude::*;

use event::{ Escalator };

#[derive(Clone, Debug)]
pub struct State {
	pub pfp: gtk::Image,
	pub label: gtk::Label,
	pub inner: gtk::Box,
	pub escalator: Escalator,
}

impl State {
	pub fn new(data: &data::State, parent: &gtk::Box, escalator: &Escalator) -> Self {
		let ctx = gtk::Box::new(gtk::Orientation::Horizontal, 8);
		ctx.set_widget_name("ctx");

		// Yeah, I'm aware this is utter garbage. I just don't care enough 
		// to fix it. If it ain't broke, don't fix it.
		let pfp = {
			let path = Path::new("/home/")
				.join(var("USER").unwrap_or(String::from("")))
				.join(".face");
			let mut file = match File::open(&path) {
				Ok(file) => file,
				Err(e) => panic!(format!("\n\nFailed to load {:?}.\n\nDetails:\n\n{}\n\n", &path, e)),
			};
			let surface = match cairo::ImageSurface::create_from_png(&mut file) {
				Ok(surface) => surface,
				Err(e) => panic!(format!("\n\nFailed to load {:?}.\n\nDetails:\n\n{}\n\n", &path, e)),
			};
			let pfp = gtk::Image::from_file(&path);
			pfp.set_widget_name("pfp");
			pfp.connect_draw(move |_, ctx| -> Inhibit {
				ctx.set_source_surface(&surface, 0.0, 0.0);
				ctx.arc(16.0, 16.0, 16.0, 0.0, 2.0 * std::f64::consts::PI);
				ctx.clip();
				ctx.paint();
				Inhibit(true)
			});
			pfp
		};
		ctx.add(&pfp);

		let label = gtk::Label::new(None);
		label.set_markup(
			&format!(r##"<b><span foreground="#cccccc">{user}</span><span foreground="#4285f4">@{host}</span></b>"##,
				user = data.context.user.clone(),
				host = data.context.host.clone()));
		ctx.add(&label);
		
		parent.add(&ctx);

		State {
			pfp,
			label,
			inner: ctx,
			escalator: escalator.clone(),
		}
	}
}
