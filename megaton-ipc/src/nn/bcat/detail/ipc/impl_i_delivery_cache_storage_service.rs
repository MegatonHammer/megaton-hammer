
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IDeliveryCacheStorageService<T>(T);

impl IDeliveryCacheStorageService<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IDeliveryCacheStorageService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IDeliveryCacheStorageService(domain)),
			Err((sess, err)) => Err((IDeliveryCacheStorageService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IDeliveryCacheStorageService<Session>> {
		Ok(IDeliveryCacheStorageService(self.0.duplicate()?))
	}
}

impl<T> Deref for IDeliveryCacheStorageService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IDeliveryCacheStorageService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IDeliveryCacheStorageService<T> {
	pub fn create_file_service(&self, ) -> Result<::nn::bcat::detail::ipc::IDeliveryCacheFileService<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn create_directory_service(&self, ) -> Result<::nn::bcat::detail::ipc::IDeliveryCacheDirectoryService<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn enumerate_delivery_cache_directory(&self, unk1: &mut [::nn::bcat::DirectoryName]) -> Result<i32> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(10)
			.args(())
			.descriptor(IPCBuffer::from_mut_slice(unk1, 6))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl<T: Object> From<T> for IDeliveryCacheStorageService<T> {
	fn from(obj: T) -> IDeliveryCacheStorageService<T> {
		IDeliveryCacheStorageService(obj)
	}
}
