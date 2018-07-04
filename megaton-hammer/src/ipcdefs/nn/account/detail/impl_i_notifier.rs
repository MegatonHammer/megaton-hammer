
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct INotifier<T>(T);

impl INotifier<Session> {
	pub fn to_domain(self) -> ::core::result::Result<INotifier<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(INotifier(domain)),
			Err((sess, err)) => Err((INotifier(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<INotifier<Session>> {
		Ok(INotifier(self.0.duplicate()?))
	}
}

impl<T> Deref for INotifier<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for INotifier<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> INotifier<T> {
	pub fn get_system_event(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

}

impl<T: Object> From<T> for INotifier<T> {
	fn from(obj: T) -> INotifier<T> {
		INotifier(obj)
	}
}
