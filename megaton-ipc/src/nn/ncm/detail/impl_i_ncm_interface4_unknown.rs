
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct INcmInterface4Unknown<T>(T);

impl INcmInterface4Unknown<Session> {
	pub fn to_domain(self) -> ::core::result::Result<INcmInterface4Unknown<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(INcmInterface4Unknown(domain)),
			Err((sess, err)) => Err((INcmInterface4Unknown(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<INcmInterface4Unknown<Session>> {
		Ok(INcmInterface4Unknown(self.0.duplicate()?))
	}
}

impl<T> Deref for INcmInterface4Unknown<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for INcmInterface4Unknown<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> INcmInterface4Unknown<T> {
	pub fn unknown10(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(10)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown13(&self, ) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(13)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl<T: Object> From<T> for INcmInterface4Unknown<T> {
	fn from(obj: T) -> INcmInterface4Unknown<T> {
		INcmInterface4Unknown(obj)
	}
}
