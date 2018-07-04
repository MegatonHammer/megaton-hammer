
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IReceiver<T>(T);

impl IReceiver<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IReceiver<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IReceiver(domain)),
			Err((sess, err)) => Err((IReceiver(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IReceiver<Session>> {
		Ok(IReceiver(self.0.duplicate()?))
	}
}

impl<T> Deref for IReceiver<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IReceiver<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IReceiver<T> {
	pub fn unknown0(&self, unk0: u128) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown1(&self, unk0: u128) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown2(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn unknown3(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown4(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IReceiver<T> {
	fn from(obj: T) -> IReceiver<T> {
		IReceiver(obj)
	}
}
