use crate::gtk;

use crate::event;

use gtk::prelude::*;

use event::{ Escalator, Event };

#[derive(Clone, Debug)]
pub struct State ( gtk::Box, );

pub fn actions(escalator: &Escalator) -> gtk::Box {
	let actions = gtk::Box::new(gtk::Orientation::Horizontal, 0);
	actions.set_halign(gtk::Align::Center);
	actions.add(&action("󰒓", "Open settings",   Event::OpenSettings,        &escalator));
	actions.add(&action("󰌾", "Lock the screen", Event::LockScreen,          &escalator));
	actions.add(&action("󰍃", "Leave",           Event::OpenLeaveConfirm,    &escalator));
	actions.add(&action("󰐥", "Shut down",       Event::OpenShutDownConfirm, &escalator));
	actions
}

fn action(label: &str, hover: &str, event: Event, escalator: &Escalator) -> gtk::Box {
	let escalator_clone = escalator.clone();
	let settings_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
	settings_box.set_size_request(50, 0);
	{
		let settings_button = gtk::Button::new();
		settings_button.set_tooltip_text(Some(hover));
		settings_button.set_size_request(32, 32);
		settings_button.set_widget_name("action-button");
		settings_button.connect_clicked(move |_| escalator_clone.escalate(event.clone()));
		{
			let settings_icon = gtk::Label::new(None);
			settings_icon.set_markup(&format!(r#"<span size="large">{}</span>"#, label));
			settings_button.add(&settings_icon);
		}
		settings_box.add(&settings_button);
	}
	settings_box
}
