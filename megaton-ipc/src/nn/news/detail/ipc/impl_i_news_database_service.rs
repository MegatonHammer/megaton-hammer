
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct INewsDatabaseService<T>(T);

impl INewsDatabaseService<Session> {
	pub fn to_domain(self) -> ::core::result::Result<INewsDatabaseService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(INewsDatabaseService(domain)),
			Err((sess, err)) => Err((INewsDatabaseService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<INewsDatabaseService<Session>> {
		Ok(INewsDatabaseService(self.0.duplicate()?))
	}
}

impl<T> Deref for INewsDatabaseService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for INewsDatabaseService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> INewsDatabaseService<T> {
	// fn unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown3(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown4(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown5(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for INewsDatabaseService<T> {
	fn from(obj: T) -> INewsDatabaseService<T> {
		INewsDatabaseService(obj)
	}
}
