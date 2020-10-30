#[macro_use] pub extern crate lazy_static;
pub extern crate cairo;
pub extern crate chrono;
pub extern crate crossbeam;
pub extern crate clap;
pub extern crate log;
pub extern crate gdk;
pub extern crate gio;
pub extern crate glib;
pub extern crate gtk;
pub extern crate interpolation;
pub extern crate pango;
pub extern crate x11rb;

pub mod event;
pub mod freedesktop;
pub mod state;
pub mod ui;
pub mod logger;

use std::env::{ args };
use std::thread::{ sleep };
use std::time::{ Duration };

use gio::prelude::*;
use gtk::prelude::*;
use log::{ LevelFilter, set_logger, set_max_level, trace };

use state::{ State };
use ui::{ CSS };
use logger::{ LOGGER };

fn main() {
    // Set up logging.
    set_logger(&LOGGER).expect("\n\nFailed to initialize logging.\n\n");
    set_max_level(LevelFilter::max());
    trace!("Logging initialized.");

    trace!("Initializing Clap...");
    let matches = clap::App::new("launcher")
        .arg(clap::Arg::with_name("x")
            .short("x")
            .long("x")
            .value_name("X_POSITION")
            .validator(|v| -> Result<(), String> {
                match v.parse::<i32>() {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("{}", e)),
                }
            }))
        .get_matches();

    // Create a new `gtk::Application`.
    trace!("Initializing GTK...");
    let gtk_app = {
        let try_gtk_app = gtk::Application::new(
            Some("oss.k4rakara.launcher"),
            gio::ApplicationFlags::FLAGS_NONE);
        match try_gtk_app {
            Ok(gtk_app) => gtk_app,
            Err(e) => panic!(e),
        }
    };

    gtk_app.connect_activate(move |app| {
		let win = gtk::ApplicationWindowBuilder::new()
			.application(app)
			.type_(gtk::WindowType::Popup)
			.title("launcher")
			.default_width(400)
			.default_height(0)
			.decorated(false)
			.build();

		// Set window attributes.
		win.set_keep_above(true);
		win.stick();
        win.set_opacity(0.0);

        // Create state.
        let mut state = State::new(&app);

        // Load CSS.
        let provider = gtk::CssProvider::new();
        provider.load_from_data(CSS.as_bytes()).expect("\n\nFailed to load CSS.\n\n");
        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().expect("\n\nFailed to initialize default GTK CSS provider.\n\n"),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
		);
		
		win.show_all();

        glib::idle_add_local(move || {
            state.update();
			sleep(Duration::from_millis(8));
            glib::Continue(true)
        });

        trace!("Initialized GTK.");
    });


    gtk_app.run(&args().collect::<Vec<_>>());
}

