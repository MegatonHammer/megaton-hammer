
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IRegistrar<T>(T);

impl IRegistrar<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IRegistrar<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IRegistrar(domain)),
			Err((sess, err)) => Err((IRegistrar(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IRegistrar<Session>> {
		Ok(IRegistrar(self.0.duplicate()?))
	}
}

impl<T> Deref for IRegistrar<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IRegistrar<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IRegistrar<T> {
	pub fn issue(&self, unk0: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_application_launch_property(&self, unk0: u128) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn set_application_control_property(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IRegistrar<T> {
	fn from(obj: T) -> IRegistrar<T> {
		IRegistrar(obj)
	}
}
