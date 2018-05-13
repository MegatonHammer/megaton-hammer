
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct ISaveDataInfoReader<T>(T);

impl ISaveDataInfoReader<Session> {
	pub fn to_domain(self) -> ::core::result::Result<ISaveDataInfoReader<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ISaveDataInfoReader(domain)),
			Err((sess, err)) => Err((ISaveDataInfoReader(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ISaveDataInfoReader<Session>> {
		Ok(ISaveDataInfoReader(self.0.duplicate()?))
	}
}

impl<T> Deref for ISaveDataInfoReader<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ISaveDataInfoReader<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ISaveDataInfoReader<T> {
	// fn unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for ISaveDataInfoReader<T> {
	fn from(obj: T) -> ISaveDataInfoReader<T> {
		ISaveDataInfoReader(obj)
	}
}
