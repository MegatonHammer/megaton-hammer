
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct INcmInterface5Unknown<T>(T);

impl INcmInterface5Unknown<Session> {
	pub fn to_domain(self) -> ::core::result::Result<INcmInterface5Unknown<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(INcmInterface5Unknown(domain)),
			Err((sess, err)) => Err((INcmInterface5Unknown(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<INcmInterface5Unknown<Session>> {
		Ok(INcmInterface5Unknown(self.0.duplicate()?))
	}
}

impl<T> Deref for INcmInterface5Unknown<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for INcmInterface5Unknown<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> INcmInterface5Unknown<T> {
	pub fn unknown5(&self, ) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(5)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn unknown7(&self, ) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(7)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn unknown8(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(8)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown15(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(15)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for INcmInterface5Unknown<T> {
	fn from(obj: T) -> INcmInterface5Unknown<T> {
		INcmInterface5Unknown(obj)
	}
}
