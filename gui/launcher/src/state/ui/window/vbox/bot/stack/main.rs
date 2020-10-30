use crate::gtk;

use crate::state::data;
use crate::state::ui::components;
use crate::event;

use gtk::prelude::*;

use event::{ Escalator };
use components::{ application_list };

pub fn main(escalator: &Escalator, data: &data::State) -> gtk::Box {
	let main = gtk::Box::new(gtk::Orientation::Vertical, 0);
	{
		let notebook = gtk::Notebook::new();
		notebook.set_widget_name("notebook");
		notebook.set_scrollable(true);
		notebook.set_vexpand(true);
		notebook.set_tab_pos(gtk::PositionType::Left);
		fn add_tab<W: IsA<gtk::Widget>>(n: &gtk::Notebook, l: &str, w: &W) {
			let tab = gtk::Box::new(gtk::Orientation::Horizontal, 0);
			{
				let label = gtk::Label::new(None);
				label.set_markup(&format!(r#"<span size="x-large">{}</span>"#, l));
				tab.set_center_widget(Some(&label));
			}
			tab.show_all();
			n.append_page(w, Some(&tab));
		}
		{
			add_tab(&notebook, "󰓎", &application_list("Favorites",        &data.applications.favorites,               &escalator));
			add_tab(&notebook, "󰥔", &application_list("Recent",           &data.applications.recent,                  &escalator));
			add_tab(&notebook, "󰀻", &application_list("All Applications", &data.applications.all,                     &escalator));
			add_tab(&notebook, "󰇥", &application_list("Development",      &data.applications.by_category.development, &escalator));
			add_tab(&notebook, "󰮂", &application_list("Games",            &data.applications.by_category.game,        &escalator));
			add_tab(&notebook, "󰃣", &application_list("Graphics",         &data.applications.by_category.graphics,    &escalator));
			add_tab(&notebook, "󰖟", &application_list("Network",          &data.applications.by_category.network,     &escalator));
			add_tab(&notebook, "󰤽", &application_list("Multimedia",       &data.applications.by_category.audiovideo,  &escalator));
			add_tab(&notebook, "󰒓", &application_list("Settings",         &data.applications.by_category.settings,    &escalator));
			add_tab(&notebook, "󰇅", &application_list("System",           &data.applications.by_category.system,      &escalator));
			add_tab(&notebook, "󰙏", &application_list("Utilities",        &data.applications.by_category.utility,     &escalator));
		}
		main.add(&notebook);
	}
	main
}

