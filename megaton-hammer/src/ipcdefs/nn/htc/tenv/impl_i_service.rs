
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IService<T>(T);

impl IService<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IService(domain)),
			Err((sess, err)) => Err((IService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IService<Session>> {
		Ok(IService(self.0.duplicate()?))
	}
}

impl<T> Deref for IService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IService<T> {
	// fn unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown2(&self, unk0: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IService<T> {
	fn from(obj: T) -> IService<T> {
		IService(obj)
	}
}
