use crate::gtk;

use crate::freedesktop;
use crate::event;

use gtk::prelude::*;

use freedesktop::{ DesktopEntry };
use event::{ Escalator, Event };

pub fn application_list(label: &str, applications: &Vec<DesktopEntry>, escalator: &Escalator) -> gtk::Box {
	let root = gtk::Box::new(gtk::Orientation::Vertical, 0);
    root.set_widget_name("application-list");
    root.set_vexpand(true);
    {
        let header_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        header_box.set_widget_name("header-box");
        {
            let header = gtk::Label::new(None);
            header.set_widget_name("header");
            header.set_markup(&format!(r#"<span size="large">{}</span>"#, label));
            header_box.add(&header);
        }
        root.add(&header_box);

        let scroller = gtk::ScrolledWindow::new::<gtk::Adjustment, gtk::Adjustment>(None, None);
        scroller.set_vexpand(true);
        {
            let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
            {
                for application in applications.iter() {
                    vbox.add(&application_button(&application, &escalator));
                }
            }
            scroller.add(&vbox);
        }
        root.add(&scroller);
    }
    root
}

pub fn application_button(a: &DesktopEntry, escalator: &Escalator) -> gtk::Button {
	let escalator_cloned = escalator.clone();
	let a_cloned = a.clone();
	let button = gtk::Button::new();
	button.set_widget_name("application-button");
	button.connect_clicked(move |_| escalator_cloned.escalate(Event::OpenApplication(a_cloned.clone())));
    {
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        {
            let image = gtk::Image::from_icon_name(Some(&a.icon), gtk::IconSize::Dnd);
            image.set_pixel_size(32);
            hbox.add(&image);

            let vbox = gtk::Box::new(gtk::Orientation::Vertical, 4);
            {
                let hbox_1 = gtk::Box::new(gtk::Orientation::Horizontal, 0);
                hbox_1.set_hexpand(true);
                {
                    let label = gtk::Label::new(None);
                    let txt = {
                        if a.name.len() > 32 {
                            let mut to_return = String::new();
                            let mut chars = a.name.chars();
                            for _ in 0..29 {
                                if let Some(this_char) = chars.next() {
                                    if this_char.is_ascii() {
                                        to_return.push(this_char);
                                    }
                                }
                            }
                            to_return = format!("{}...", to_return);
                            to_return
                        } else {
                            a.name.clone()
                        }
                    };
                    label.set_markup(&format!("{}", txt.replace("&", "&amp;")));
                    hbox_1.add(&label);
                }
                vbox.add(&hbox_1);

                let hbox_2 = gtk::Box::new(gtk::Orientation::Horizontal, 0);
                hbox_2.set_hexpand(true);
                {
                    let description = gtk::Label::new(None);
                    let txt = {
                        if a.description.len() > 48 {
                            let mut to_return = String::new();
                            let mut chars = a.description.chars();
                            for _ in 0..45 {
                                if let Some(this_char) = chars.next() {
                                    to_return.push(this_char);
                                }
                            }
                            to_return = format!("{}...", to_return);
                            to_return
                        } else {
                            a.description.clone()
                        }
                    };
                    description.set_markup(&format!(r#"<span size="x-small">{}</span>"#, txt));
                    hbox_2.add(&description);
                }
                vbox.add(&hbox_2);
            }
            hbox.add(&vbox);
        }
        button.add(&hbox);
    }
    button
}
