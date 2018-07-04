
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct ILogger<T>(T);

impl ILogger<Session> {
	pub fn to_domain(self) -> ::core::result::Result<ILogger<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ILogger(domain)),
			Err((sess, err)) => Err((ILogger(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ILogger<Session>> {
		Ok(ILogger(self.0.duplicate()?))
	}
}

impl<T> Deref for ILogger<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ILogger<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ILogger<T> {
	// fn unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown1(&self, unk0: u32) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for ILogger<T> {
	fn from(obj: T) -> ILogger<T> {
		ILogger(obj)
	}
}
