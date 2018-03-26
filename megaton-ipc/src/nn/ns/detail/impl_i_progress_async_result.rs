
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IProgressAsyncResult<T>(T);

impl IProgressAsyncResult<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IProgressAsyncResult<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IProgressAsyncResult(domain)),
			Err((sess, err)) => Err((IProgressAsyncResult(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IProgressAsyncResult<Session>> {
		Ok(IProgressAsyncResult(self.0.duplicate()?))
	}
}

impl<T> Deref for IProgressAsyncResult<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IProgressAsyncResult<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IProgressAsyncResult<T> {
	pub fn unknown0(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown1(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown3(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IProgressAsyncResult<T> {
	fn from(obj: T) -> IProgressAsyncResult<T> {
		IProgressAsyncResult(obj)
	}
}
