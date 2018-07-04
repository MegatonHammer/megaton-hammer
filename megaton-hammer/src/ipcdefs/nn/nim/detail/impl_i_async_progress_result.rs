
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IAsyncProgressResult<T>(T);

impl IAsyncProgressResult<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IAsyncProgressResult<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IAsyncProgressResult(domain)),
			Err((sess, err)) => Err((IAsyncProgressResult(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IAsyncProgressResult<Session>> {
		Ok(IAsyncProgressResult(self.0.duplicate()?))
	}
}

impl<T> Deref for IAsyncProgressResult<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IAsyncProgressResult<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IAsyncProgressResult<T> {
	pub fn unknown0(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown1(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown2(&self, ) -> Result<u128> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let res : Response<u128> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl<T: Object> From<T> for IAsyncProgressResult<T> {
	fn from(obj: T) -> IAsyncProgressResult<T> {
		IAsyncProgressResult(obj)
	}
}
