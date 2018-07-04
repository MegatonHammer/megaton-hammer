
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IReadSession<T>(T);

impl IReadSession<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IReadSession<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IReadSession(domain)),
			Err((sess, err)) => Err((IReadSession(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IReadSession<Session>> {
		Ok(IReadSession(self.0.duplicate()?))
	}
}

impl<T> Deref for IReadSession<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IReadSession<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IReadSession<T> {
	// fn unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IReadSession<T> {
	fn from(obj: T) -> IReadSession<T> {
		IReadSession(obj)
	}
}
