use crate::crossbeam;

use crate::log;
use crate::gtk;

use crate::event;
pub mod data;
pub mod ui;

use std::time::{ Duration };
use std::env::{ vars_os };
use std::process::{ Command, exit, Stdio };

use gtk::prelude::*;
use crossbeam::channel::{ Receiver };
use log::{ error, info, trace, warn };

use event::{ Escalator, Event };

#[derive(Clone, Debug)]
pub struct State {
	pub app: gtk::Application,
    pub data: data::State,
	pub ui: ui::State,
	pub escalator: (Escalator, Receiver<Event>),
}

impl State {
	pub fn new(app: &gtk::Application) -> Self {
		let escalator = Escalator::new();
		let data = data::State::new();
		let ui = ui::State::new(&data, &app, &escalator.0);
		State {
            app: app.clone(),
			data,
			ui,
			escalator,
		}
	}

	pub fn update(&mut self) {
		self.ui.update();
		while !self.escalator.1.is_empty() {
			match self.escalator.1.recv() {
				Ok(e) => {
					match e {
                        Event::OpenSettings => {
                            let cmd = "xfce4-settings-manager";
                            info!("Starting {cmd:?}...", cmd = &cmd);
                            let try_spawn = Command::new("sh")
                                .arg("-c")
                                .arg(&cmd)
                                .envs(&mut vars_os())
                                .stdout(Stdio::null())
                                .stderr(Stdio::null())
                                .stdin(Stdio::null())
                                .spawn();
                            match try_spawn {
                                Ok(_) => {
                                    info!("Started {cmd:?}.", cmd = &cmd);
    
                                    self.escalator.0.escalate(Event::Close);

                                    let cloned_escalator = self.escalator.0.clone();
                                    glib::timeout_add(500, move || -> glib::Continue {
                                        cloned_escalator.escalate(Event::Exit);
                                        glib::Continue(false)
                                    });
                                }
                                Err(e) => warn!("Failed to start {cmd:?}. Details: {err}",
                                    cmd = &cmd,
                                    err = e),
                            }
                        }
                        Event::LockScreen => {
                            
                        }
                        Event::OpenLeaveConfirm => {
                            self.escalator.0.escalate(Event::Close);

                            let cloned_escalator = self.escalator.0.clone();
                            glib::timeout_add_local(500, move || -> glib::Continue {
                                let dialog = gtk::MessageDialog::new::<gtk::Window>(
                                    None,
                                    gtk::DialogFlags::empty(),
                                    gtk::MessageType::Question,
                                    gtk::ButtonsType::YesNo,
                                    "Are you sure you want to sign out?");
                                match dialog.run() {
									gtk::ResponseType::Yes => {
										cloned_escalator.escalate(Event::Exit);
										let try_spawn = Command::new("sh")
											.arg("-c")
											.arg("pkill xinit")
											.envs(&mut vars_os())
											.stdout(Stdio::null())
											.stderr(Stdio::null())
											.stdin(Stdio::null())
											.spawn();
										match try_spawn {
											Ok(_) => (),
											Err(e) => warn!("Failed to start \"pkill xinit\". Details: {}", e),
										}
									}
									gtk::ResponseType::No => {
										cloned_escalator.escalate(Event::Exit);
									}
									_ => (),
								}
                                glib::Continue(false)
                            });
                        }
                        Event::OpenShutDownConfirm => {
                            self.escalator.0.escalate(Event::Close);

                            let cloned_escalator = self.escalator.0.clone();
                            glib::timeout_add_local(500, move || -> glib::Continue {
                                let dialog = gtk::MessageDialog::new::<gtk::Window>(
                                    None,
                                    gtk::DialogFlags::empty(),
                                    gtk::MessageType::Question,
                                    gtk::ButtonsType::YesNo,
                                    "Are you sure you want to shut down?");
                                match dialog.run() {
                                    gtk::ResponseType::Yes => {
                                        // TODO: Create a custom shut down options screen.
                                        let try_spawn = Command::new("sh")
                                            .arg("-c")
                                            .arg("sudo shutdown -h now")
                                            .envs(&mut vars_os())
                                            .stdout(Stdio::null())
                                            .stderr(Stdio::null())
                                            .stdin(Stdio::null())
                                            .spawn();
                                        match try_spawn {
                                            Ok(_) => (),
                                            Err(e) => error!("Failed to run \"sudo shutdown -h now\". Details: {}", e),
                                        }
                                        cloned_escalator.escalate(Event::Exit);
                                    }
                                    gtk::ResponseType::No => {
                                        cloned_escalator.escalate(Event::Exit);
                                    }
                                    _ => (),
                                }
                                glib::Continue(false)
                            });
                        }
						Event::OpenApplication(mut app) => {
                            app.exec = String::from(app.exec.replace("%u", "").replace("%U", "").replace("%f", "").trim());
							info!("Starting {cmd:?}...", cmd = &app.exec);
                            let try_spawn = Command::new("sh")
                                .arg("-c")
                                .arg(&app.exec)
                                .envs(&mut vars_os())
                                .stdout(Stdio::null())
                                .stderr(Stdio::null())
                                .stdin(Stdio::null())
                                .spawn();
                            match try_spawn {
                                Ok(_) => {
                                    info!("Started {cmd:?}.", cmd = &app.exec);
                                   
                                    self.escalator.0.escalate(Event::Close);
                                    
                                    let cloned_escalator = self.escalator.0.clone();
                                    glib::timeout_add(500, move || -> glib::Continue {
                                        cloned_escalator.escalate(Event::Exit);
                                        glib::Continue(false)
                                    });
                                }
                                Err(e) => warn!("Failed to start {cmd:?}. Details: {err}",
                                    cmd = &app.exec,
                                    err = e),
                            }
						}
                        Event::Close => {
                            trace!("Playing window animation...");
                            self.ui.window2.close();
                            self.ui.window.animation = Some(
                                ui::window::Animation::new(
                                    self.ui.window.width, self.ui.window.height,
                                    self.ui.window.width, 1,
                                    1.0, -1.0,
                                    self.ui.window.screen_height,
                                    Duration::from_millis(500)));
                        }
                        Event::Exit => {
                            trace!("Exiting...");
                            exit(0);
                        }
					}
				}
				Err(_) => (),
			}
		}
	}
}
