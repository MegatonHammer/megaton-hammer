
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IOverwriteEventHolder<T>(T);

impl IOverwriteEventHolder<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IOverwriteEventHolder<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IOverwriteEventHolder(domain)),
			Err((sess, err)) => Err((IOverwriteEventHolder(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IOverwriteEventHolder<Session>> {
		Ok(IOverwriteEventHolder(self.0.duplicate()?))
	}
}

impl<T> Deref for IOverwriteEventHolder<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IOverwriteEventHolder<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IOverwriteEventHolder<T> {
	pub fn unknown0(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

}

impl<T: Object> From<T> for IOverwriteEventHolder<T> {
	fn from(obj: T) -> IOverwriteEventHolder<T> {
		IOverwriteEventHolder(obj)
	}
}
