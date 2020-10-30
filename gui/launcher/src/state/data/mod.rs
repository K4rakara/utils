pub mod applications;
pub mod context;
pub mod rofi;

#[derive(Clone, Debug)]
pub struct State {
	pub applications: applications::State,
    pub context: context::State,
}

impl State {
	pub fn new() -> Self {
		State {
			applications: applications::State::new(&rofi::State::new()),
			context: context::State::new(),
		}
	}
}
