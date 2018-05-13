
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IPortSession<T>(T);

impl IPortSession<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IPortSession<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IPortSession(domain)),
			Err((sess, err)) => Err((IPortSession(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IPortSession<Session>> {
		Ok(IPortSession(self.0.duplicate()?))
	}
}

impl<T> Deref for IPortSession<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IPortSession<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IPortSession<T> {
	// fn open_session(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn open_session_for_test(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown2(&self, ) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn unknown3(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown4(&self, ) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(4)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn unknown5(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown6(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown7(&self, unk0: u32) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(7)
			.args(unk0)
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl<T: Object> From<T> for IPortSession<T> {
	fn from(obj: T) -> IPortSession<T> {
		IPortSession(obj)
	}
}
