
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IStorage<T>(T);

impl IStorage<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IStorage<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IStorage(domain)),
			Err((sess, err)) => Err((IStorage(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IStorage<Session>> {
		Ok(IStorage(self.0.duplicate()?))
	}
}

impl<T> Deref for IStorage<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IStorage<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IStorage<T> {
	pub fn open(&self, ) -> Result<::nn::am::service::IStorageAccessor<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn open_transfer_storage(&self, ) -> Result<::nn::am::service::ITransferStorageAccessor<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

}

impl<T: Object> From<T> for IStorage<T> {
	fn from(obj: T) -> IStorage<T> {
		IStorage(obj)
	}
}
