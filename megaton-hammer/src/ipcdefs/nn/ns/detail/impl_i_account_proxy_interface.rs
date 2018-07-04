
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IAccountProxyInterface<T>(T);

impl IAccountProxyInterface<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IAccountProxyInterface<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IAccountProxyInterface(domain)),
			Err((sess, err)) => Err((IAccountProxyInterface(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IAccountProxyInterface<Session>> {
		Ok(IAccountProxyInterface(self.0.duplicate()?))
	}
}

impl<T> Deref for IAccountProxyInterface<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IAccountProxyInterface<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IAccountProxyInterface<T> {
	// fn create_user_account(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IAccountProxyInterface<T> {
	fn from(obj: T) -> IAccountProxyInterface<T> {
		IAccountProxyInterface(obj)
	}
}
