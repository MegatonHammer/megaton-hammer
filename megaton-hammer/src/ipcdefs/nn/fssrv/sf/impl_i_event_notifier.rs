
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IEventNotifier<T>(T);

impl IEventNotifier<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IEventNotifier<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IEventNotifier(domain)),
			Err((sess, err)) => Err((IEventNotifier(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IEventNotifier<Session>> {
		Ok(IEventNotifier(self.0.duplicate()?))
	}
}

impl<T> Deref for IEventNotifier<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IEventNotifier<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IEventNotifier<T> {
	pub fn get_event_handle(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

}

impl<T: Object> From<T> for IEventNotifier<T> {
	fn from(obj: T) -> IEventNotifier<T> {
		IEventNotifier(obj)
	}
}
