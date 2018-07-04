
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct ISessionObject<T>(T);

impl ISessionObject<Session> {
	pub fn to_domain(self) -> ::core::result::Result<ISessionObject<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ISessionObject(domain)),
			Err((sess, err)) => Err((ISessionObject(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ISessionObject<Session>> {
		Ok(ISessionObject(self.0.duplicate()?))
	}
}

impl<T> Deref for ISessionObject<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ISessionObject<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ISessionObject<T> {
	pub fn dummy(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(999)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for ISessionObject<T> {
	fn from(obj: T) -> ISessionObject<T> {
		ISessionObject(obj)
	}
}
