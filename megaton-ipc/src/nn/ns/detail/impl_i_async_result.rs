
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IAsyncResult<T>(T);

impl IAsyncResult<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IAsyncResult<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IAsyncResult(domain)),
			Err((sess, err)) => Err((IAsyncResult(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IAsyncResult<Session>> {
		Ok(IAsyncResult(self.0.duplicate()?))
	}
}

impl<T> Deref for IAsyncResult<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IAsyncResult<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IAsyncResult<T> {
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

}

impl<T: Object> From<T> for IAsyncResult<T> {
	fn from(obj: T) -> IAsyncResult<T> {
		IAsyncResult(obj)
	}
}
