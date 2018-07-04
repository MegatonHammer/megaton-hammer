
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IClientIfSession<T>(T);

impl IClientIfSession<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IClientIfSession<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IClientIfSession(domain)),
			Err((sess, err)) => Err((IClientIfSession(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IClientIfSession<Session>> {
		Ok(IClientIfSession(self.0.duplicate()?))
	}
}

impl<T> Deref for IClientIfSession<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IClientIfSession<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IClientIfSession<T> {
	pub fn unknown0(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown3(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown4(&self, ) -> Result<u32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(4)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn unknown5(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown6(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(6)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn unknown7(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown8(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(8)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown9(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IClientIfSession<T> {
	fn from(obj: T) -> IClientIfSession<T> {
		IClientIfSession(obj)
	}
}
