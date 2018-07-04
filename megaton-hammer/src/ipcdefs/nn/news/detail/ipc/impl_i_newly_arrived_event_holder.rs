
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct INewlyArrivedEventHolder<T>(T);

impl INewlyArrivedEventHolder<Session> {
	pub fn to_domain(self) -> ::core::result::Result<INewlyArrivedEventHolder<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(INewlyArrivedEventHolder(domain)),
			Err((sess, err)) => Err((INewlyArrivedEventHolder(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<INewlyArrivedEventHolder<Session>> {
		Ok(INewlyArrivedEventHolder(self.0.duplicate()?))
	}
}

impl<T> Deref for INewlyArrivedEventHolder<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for INewlyArrivedEventHolder<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> INewlyArrivedEventHolder<T> {
	pub fn unknown0(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

}

impl<T: Object> From<T> for INewlyArrivedEventHolder<T> {
	fn from(obj: T) -> INewlyArrivedEventHolder<T> {
		INewlyArrivedEventHolder(obj)
	}
}
