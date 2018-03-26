
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IStorageAccessor<T>(T);

impl IStorageAccessor<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IStorageAccessor<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IStorageAccessor(domain)),
			Err((sess, err)) => Err((IStorageAccessor(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IStorageAccessor<Session>> {
		Ok(IStorageAccessor(self.0.duplicate()?))
	}
}

impl<T> Deref for IStorageAccessor<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IStorageAccessor<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IStorageAccessor<T> {
	pub fn get_size(&self, ) -> Result<i64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn write(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn read(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IStorageAccessor<T> {
	fn from(obj: T) -> IStorageAccessor<T> {
		IStorageAccessor(obj)
	}
}
