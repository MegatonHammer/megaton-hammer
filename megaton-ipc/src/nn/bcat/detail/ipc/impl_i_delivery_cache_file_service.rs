
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IDeliveryCacheFileService<T>(T);

impl IDeliveryCacheFileService<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IDeliveryCacheFileService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IDeliveryCacheFileService(domain)),
			Err((sess, err)) => Err((IDeliveryCacheFileService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IDeliveryCacheFileService<Session>> {
		Ok(IDeliveryCacheFileService(self.0.duplicate()?))
	}
}

impl<T> Deref for IDeliveryCacheFileService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IDeliveryCacheFileService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IDeliveryCacheFileService<T> {
	pub fn open(&self, unk0: ::nn::bcat::DirectoryName, unk1: ::nn::bcat::FileName) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::bcat::DirectoryName,
			unk1: ::nn::bcat::FileName,
		}
		let req = Request::new(0)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn read(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_size(&self, ) -> Result<i64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_digest(&self, ) -> Result<::nn::bcat::Digest> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			;
		let res : Response<::nn::bcat::Digest> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl<T: Object> From<T> for IDeliveryCacheFileService<T> {
	fn from(obj: T) -> IDeliveryCacheFileService<T> {
		IDeliveryCacheFileService(obj)
	}
}
