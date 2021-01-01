use crate::gtk;
use crate::log;
use crate::interpolation;
use crate::regex;

use crate::state::data;
use crate::event;
pub mod vbox;

use std::process::{ Command };
use std::time::{ Duration, Instant };

use gtk::prelude::*;
use log::{ trace };
use interpolation::{ Ease, lerp };
use regex::{ Regex };

use event::{ Escalator };

#[derive(Clone, Debug)]
pub struct State {
	pub inner: gtk::Window,	
    pub vbox: vbox::State,
	pub x: i32,
	pub y: i32,
	pub width: i32,
	pub height: i32,
	pub screen_width: u16,
	pub screen_height: u16,
    pub opacity: f32,
    pub animation: Option<Animation>,
}

impl State {
	pub fn new(data: &data::State, win: &gtk::Window, escalator: &Escalator) -> Self {
        let (s_off_x, s_off_y, s_width, s_height) = (|| {
            let try_output = Command::new("xrandr")
                .arg("-q")
                .output();
            
            let output = match try_output {
                Ok(output) => output,
                Err(error) => panic!(format!("\n\nFailed to run \"xrandr -q\".\n\nDetails:\n\n{}\n\n", error)),
            };
            
            let stdout = match String::from_utf8(output.stdout) {
                Ok(stdout) => stdout,
                Err(error) => panic!(format!("\n\nFailed to convert a `Vec<u8>` to a `String`.\n\nDetails:\n\n{}\n\n", error)),
            };
            
            let regex = Regex::new(r#"(?i)([a-z0-9\-_]+) connected(?: primary)? (\d+)x(\d+)\+(\d+)\+(\d+)"#).unwrap();
           
            for line in stdout.lines() {
                if regex.is_match(line) {
                    let caps = regex.captures(line).unwrap();
                    let width = caps
                        .get(2)
                        .unwrap()
                        .as_str()
                        .parse::<u16>()
                        .unwrap();
                    let height = caps
                        .get(3)
                        .unwrap()
                        .as_str()
                        .parse::<u16>()
                        .unwrap();
                    let off_x = caps
                        .get(4)
                        .unwrap()
                        .as_str()
                        .parse::<u16>()
                        .unwrap();
                    let off_y = caps
                        .get(4)
                        .unwrap()
                        .as_str()
                        .parse::<u16>()
                        .unwrap();
                    return (off_x, off_y, width, height);
                }
            }
            
            panic!(String::from("\n\nNo display connected!\n\n"));
        })();
        
		let width = ((s_width as f32 / 100.0) * 25.0).round() as i32;
		let height = 0;
        
		let x = (s_off_x + 8) as i32;
		let y = (s_off_y + s_height) as i32 - 48 - height;
        
		State {
			inner: win.clone(),
			vbox: vbox::State::new(&data, &win, &escalator),
			x: x,
			y: y,
			width: width,
			height: height,
			screen_width: s_width,
			screen_height: s_height,
            opacity: 0.0,
			animation: Some(
				Animation::new(
					width, 0,
					width, 400,
                    -1.0, 1.0,
					s_height,
					Duration::from_millis(500))),
		}
	}

	pub fn update(&mut self) {
		if self.animation.is_some() {
			let remove;
			self.animation = Some(match self.animation.clone() {
				Some(mut animation) => {
					animation.update();
					remove = animation.complete;
					animation
				}
				None => unreachable!(),
			});
			if !remove {
				let animation = self.animation.clone().unwrap();
				self.inner.resize(
					animation.width as i32,
					animation.height as i32);
				self.inner.move_(
					animation.x as i32,
					animation.y as i32);
                self.inner.set_opacity(animation.opacity as f64);
			} else {
				let animation = self.animation.clone().unwrap();
				self.x = animation.x as i32;
				self.y = animation.y as i32;
				self.width = animation.width as i32;
				self.height = animation.height as i32;
				self.inner.set_opacity(animation.opacity as f64);
                self.animation = None;
                trace!("Window animation completed.");
			}
		}
	}
}

#[derive(Clone, Debug)]
pub struct Animation {
	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32,
    pub opacity: f32,
	pub initial_width: f32,
	pub initial_height: f32,
	pub target_width: f32,
	pub target_height: f32,
    pub initial_opacity: f32,
    pub target_opacity: f32,
	pub start: Instant,
	pub duration: Duration,
	pub screen_height: i32,
	pub complete: bool,
}

impl Animation {
	pub fn new(
		width_1: i32,
		height_1: i32,
		width_2: i32,
		height_2: i32,
        opacity_1: f32,
        opacity_2: f32,
		screen_height: u16,
		duration: Duration,
	) -> Self {
		Animation {
			x: 8.0,
			y: (screen_height as i32 - 48 - height_1) as f32,
			width: width_1 as f32,
			height: height_1 as f32,
            opacity: opacity_1,
			initial_width: width_1 as f32,
			initial_height: height_1 as f32,
			target_width: width_2 as f32,
			target_height: height_2 as f32,
            initial_opacity: opacity_1,
            target_opacity: opacity_2,
			start: Instant::now(),
			duration: duration,
			screen_height: screen_height as i32,
			complete: false,
		}
	}

	pub fn update(&mut self) {
		if !self.complete {
			let progress = self.start.elapsed().as_secs_f32()
							/ self.duration.as_secs_f32();
			
			self.width = lerp(
				&self.initial_width,
				&self.target_width,
				&progress.cubic_out());
			self.height = lerp(
				&self.initial_height,
				&self.target_height,
				&progress.cubic_out());
            self.opacity = lerp(
                &self.initial_opacity,
                &self.target_opacity,
                &progress.cubic_out());
			
			self.x = 8.0;
			self.y = self.screen_height as f32 - 48.0 - self.height;
			
			if progress >= 1.0 {
				self.width = self.target_width;
				self.height = self.target_height;
                self.opacity = self.target_opacity;
				
				self.x = 8.0;
				self.y = self.screen_height as f32 - 48.0 - self.height;

				self.complete = true;
			}

            self.opacity =
                if self.opacity >= 1.0 { 1.0 }
                else if self.opacity <= 0.0 { 0.0 }
                else { self.opacity };
		}
	}
}

