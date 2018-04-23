
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IAsyncValue<T>(T);

impl IAsyncValue<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IAsyncValue<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IAsyncValue(domain)),
			Err((sess, err)) => Err((IAsyncValue(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IAsyncValue<Session>> {
		Ok(IAsyncValue(self.0.duplicate()?))
	}
}

impl<T> Deref for IAsyncValue<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IAsyncValue<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IAsyncValue<T> {
	pub fn unknown0(&self, ) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown2(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IAsyncValue<T> {
	fn from(obj: T) -> IAsyncValue<T> {
		IAsyncValue(obj)
	}
}
