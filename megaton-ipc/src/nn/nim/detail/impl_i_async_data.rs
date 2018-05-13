
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IAsyncData<T>(T);

impl IAsyncData<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IAsyncData<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IAsyncData(domain)),
			Err((sess, err)) => Err((IAsyncData(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IAsyncData<Session>> {
		Ok(IAsyncData(self.0.duplicate()?))
	}
}

impl<T> Deref for IAsyncData<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IAsyncData<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IAsyncData<T> {
	pub fn unknown0(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown1(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown2(&self, ) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn unknown3(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown4(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IAsyncData<T> {
	fn from(obj: T) -> IAsyncData<T> {
		IAsyncData(obj)
	}
}
