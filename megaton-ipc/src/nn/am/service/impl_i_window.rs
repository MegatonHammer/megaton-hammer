
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IWindow<T>(T);

impl IWindow<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IWindow<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IWindow(domain)),
			Err((sess, err)) => Err((IWindow(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IWindow<Session>> {
		Ok(IWindow(self.0.duplicate()?))
	}
}

impl<T> Deref for IWindow<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IWindow<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IWindow<T> {
	pub fn unknown12345(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(12345)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IWindow<T> {
	fn from(obj: T) -> IWindow<T> {
		IWindow(obj)
	}
}
