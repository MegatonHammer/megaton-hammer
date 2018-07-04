
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IUser<T>(T);

impl IUser<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IUser<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IUser(domain)),
			Err((sess, err)) => Err((IUser(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IUser<Session>> {
		Ok(IUser(self.0.duplicate()?))
	}
}

impl<T> Deref for IUser<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IUser<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IUser<T> {
	// fn unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown1(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown2(&self, ) -> Result<u32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn unknown3(&self, ) -> Result<u8> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(3)
			.args(())
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl<T: Object> From<T> for IUser<T> {
	fn from(obj: T) -> IUser<T> {
		IUser(obj)
	}
}
