
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IAppletResource<T>(T);

impl IAppletResource<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IAppletResource<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IAppletResource(domain)),
			Err((sess, err)) => Err((IAppletResource(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IAppletResource<Session>> {
		Ok(IAppletResource(self.0.duplicate()?))
	}
}

impl<T> Deref for IAppletResource<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IAppletResource<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IAppletResource<T> {
	pub fn get_shared_memory_handle(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

}

impl<T: Object> From<T> for IAppletResource<T> {
	fn from(obj: T) -> IAppletResource<T> {
		IAppletResource(obj)
	}
}
