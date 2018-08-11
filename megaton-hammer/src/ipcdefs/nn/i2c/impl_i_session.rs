
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct ISession<T>(T);

impl ISession<Session> {
	pub fn to_domain(self) -> ::core::result::Result<ISession<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ISession(domain)),
			Err((sess, err)) => Err((ISession(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ISession<Session>> {
		Ok(ISession(self.0.duplicate()?))
	}
}

impl<T> Deref for ISession<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ISession<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ISession<T> {
	// fn send(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn receive(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn execute_command_list(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn send_auto(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn receive_auto(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn execute_command_list_auto(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for ISession<T> {
	fn from(obj: T) -> ISession<T> {
		ISession(obj)
	}
}
