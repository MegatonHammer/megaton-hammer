
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IPmModule<T>(T);

impl IPmModule<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IPmModule<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IPmModule(domain)),
			Err((sess, err)) => Err((IPmModule(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IPmModule<Session>> {
		Ok(IPmModule(self.0.duplicate()?))
	}
}

impl<T> Deref for IPmModule<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IPmModule<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IPmModule<T> {
	// fn unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IPmModule<T> {
	fn from(obj: T) -> IPmModule<T> {
		IPmModule(obj)
	}
}
