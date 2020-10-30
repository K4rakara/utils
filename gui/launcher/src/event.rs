use crate::crossbeam;

use crate::freedesktop;

use crossbeam::channel::{ unbounded, Sender, Receiver };

use freedesktop::{ DesktopEntry };

#[derive(Clone, Debug)]
pub enum Event {
	OpenSettings,
	LockScreen,
	OpenLeaveConfirm,
	OpenShutDownConfirm,
	OpenApplication(DesktopEntry),
    Close,
    Exit,
}

/// Escalates events emitted by child components to the root.
#[derive(Clone, Debug)]
pub struct Escalator ( Sender<Event> );

impl Escalator {
	pub fn new() -> (Self, Receiver<Event>) {
		let (sender, receiver) = unbounded::<Event>();
		(Escalator(sender), receiver)
	}
	pub fn escalate(&self, e: Event) { self.0.send(e).unwrap(); }
}
