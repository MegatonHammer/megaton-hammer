
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IClientEpSession<T>(T);

impl IClientEpSession<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IClientEpSession<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IClientEpSession(domain)),
			Err((sess, err)) => Err((IClientEpSession(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IClientEpSession<Session>> {
		Ok(IClientEpSession(self.0.duplicate()?))
	}
}

impl<T> Deref for IClientEpSession<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IClientEpSession<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IClientEpSession<T> {
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

	pub fn unknown2(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn unknown3(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(3)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown4(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown5(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown6(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IClientEpSession<T> {
	fn from(obj: T) -> IClientEpSession<T> {
		IClientEpSession(obj)
	}
}
