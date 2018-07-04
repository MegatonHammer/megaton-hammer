
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct ISystem<T>(T);

impl ISystem<Session> {
	pub fn to_domain(self) -> ::core::result::Result<ISystem<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ISystem(domain)),
			Err((sess, err)) => Err((ISystem(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ISystem<Session>> {
		Ok(ISystem(self.0.duplicate()?))
	}
}

impl<T> Deref for ISystem<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ISystem<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ISystem<T> {
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

	pub fn unknown100(&self, unk0: u8) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(100)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for ISystem<T> {
	fn from(obj: T) -> ISystem<T> {
		ISystem(obj)
	}
}
