use crate::gtk;
use crate::cairo;

use crate::event;
use crate::state::data;
pub mod components;
pub mod window;

use gtk::prelude::*;
use event::{ Escalator, Event };

#[derive(Clone, Debug)]
pub struct State {
	pub app: gtk::Application,
	pub window: window::State,
    pub window2: gtk::ApplicationWindow,
	pub escalator: Escalator,
}

impl State {
	pub fn new(data: &data::State, app: &gtk::Application, escalator: &Escalator) -> Self {
        let window = match app.get_active_window() {
			Some(win) => window::State::new(&data, &win, &escalator),
			None => panic!(format!("\n\nFailed to get the active window.\n\n")),
		};

        // This shit creates an invisible second window that is used to detect
        // when the user clicks out of the menu.
        let win2 = {
            let cloned_escalator = escalator.clone();
            let win2 = gtk::ApplicationWindowBuilder::new()
                .application(app)
                .type_(gtk::WindowType::Popup)
                .title("launcher-exit")
                .default_width(window.screen_width as i32 - 2)
                .default_height(window.screen_height as i32 - 2)
                .decorated(false)
                .build();
            win2.move_(1, 1);
            win2.set_keep_above(true);
            win2.stick();
            win2.set_app_paintable(true);
            win2.connect_draw(|_, ctx| -> gtk::Inhibit {
                ctx.set_source_rgba(1.0, 1.0, 1.0, 0.0);
                ctx.set_operator(cairo::Operator::Source);
                ctx.paint();
                gtk::Inhibit(false)
            });
            fn screen_changed(win2: &gtk::ApplicationWindow, _: Option<&gdk::Screen>) {
                if let Some(screen) = win2.get_screen() {
                    if let Some(ref visual) = screen.get_rgba_visual() {
                        win2.set_visual(Some(visual));
                    }
                }
            }
            win2.connect_screen_changed(screen_changed);
            screen_changed(&win2, None);
            win2.connect_button_press_event(move |win2, _| -> gtk::Inhibit {
                win2.close();
                cloned_escalator.escalate(Event::Close);
                let cloned_cloned_escalator = cloned_escalator.clone();
                glib::timeout_add(500, move || -> glib::Continue {
                    cloned_cloned_escalator.escalate(Event::Exit);
                    glib::Continue(false)
                });
                gtk::Inhibit(false)
            });
            win2.show_all();
            win2
        };
	    State {
			app: app.clone(),
			escalator: escalator.clone(),
			window,
            window2: win2,
		}
	}

	pub fn update(&mut self) {
		self.window.update();
	}
}
