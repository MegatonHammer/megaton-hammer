
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IEnsureNetworkClockAvailabilityService<T>(T);

impl IEnsureNetworkClockAvailabilityService<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IEnsureNetworkClockAvailabilityService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IEnsureNetworkClockAvailabilityService(domain)),
			Err((sess, err)) => Err((IEnsureNetworkClockAvailabilityService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IEnsureNetworkClockAvailabilityService<Session>> {
		Ok(IEnsureNetworkClockAvailabilityService(self.0.duplicate()?))
	}
}

impl<T> Deref for IEnsureNetworkClockAvailabilityService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IEnsureNetworkClockAvailabilityService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IEnsureNetworkClockAvailabilityService<T> {
	pub fn unknown0(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown1(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn unknown2(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown3(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(3)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown4(&self, ) -> Result<u8> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(4)
			.args(())
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn unknown5(&self, ) -> Result<u64> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(5)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl<T: Object> From<T> for IEnsureNetworkClockAvailabilityService<T> {
	fn from(obj: T) -> IEnsureNetworkClockAvailabilityService<T> {
		IEnsureNetworkClockAvailabilityService(obj)
	}
}
