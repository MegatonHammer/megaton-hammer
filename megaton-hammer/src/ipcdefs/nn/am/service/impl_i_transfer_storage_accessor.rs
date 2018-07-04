
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct ITransferStorageAccessor<T>(T);

impl ITransferStorageAccessor<Session> {
	pub fn to_domain(self) -> ::core::result::Result<ITransferStorageAccessor<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ITransferStorageAccessor(domain)),
			Err((sess, err)) => Err((ITransferStorageAccessor(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ITransferStorageAccessor<Session>> {
		Ok(ITransferStorageAccessor(self.0.duplicate()?))
	}
}

impl<T> Deref for ITransferStorageAccessor<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ITransferStorageAccessor<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ITransferStorageAccessor<T> {
	pub fn get_size(&self, ) -> Result<i64> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_handle(&self, ) -> Result<(u64, KObject)> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok((*res.get_raw(),res.pop_handle()))
	}

}

impl<T: Object> From<T> for ITransferStorageAccessor<T> {
	fn from(obj: T) -> ITransferStorageAccessor<T> {
		ITransferStorageAccessor(obj)
	}
}
