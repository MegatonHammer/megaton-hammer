
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IPipe<T>(T);

impl IPipe<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IPipe<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IPipe(domain)),
			Err((sess, err)) => Err((IPipe(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IPipe<Session>> {
		Ok(IPipe(self.0.duplicate()?))
	}
}

impl<T> Deref for IPipe<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IPipe<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IPipe<T> {
	pub fn read(&self, data: &mut [u8]) -> Result<u64> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			.descriptor(IPCBuffer::from_mut_slice(data, 6))
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn write(&self, data: &[u8]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			.descriptor(IPCBuffer::from_slice(data, 5))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IPipe<T> {
	fn from(obj: T) -> IPipe<T> {
		IPipe(obj)
	}
}
