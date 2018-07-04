
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IFactoryResetInterface<T>(T);

impl IFactoryResetInterface<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IFactoryResetInterface<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IFactoryResetInterface(domain)),
			Err((sess, err)) => Err((IFactoryResetInterface(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IFactoryResetInterface<Session>> {
		Ok(IFactoryResetInterface(self.0.duplicate()?))
	}
}

impl<T> Deref for IFactoryResetInterface<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IFactoryResetInterface<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IFactoryResetInterface<T> {
	pub fn reset_to_factory_settings(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(100)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn reset_to_factory_settings_without_user_save_data(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(101)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn reset_to_factory_settings_for_refurbishment(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(102)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IFactoryResetInterface<T> {
	fn from(obj: T) -> IFactoryResetInterface<T> {
		IFactoryResetInterface(obj)
	}
}
