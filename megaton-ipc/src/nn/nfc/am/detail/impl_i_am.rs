
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IAm<T>(T);

impl IAm<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IAm<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IAm(domain)),
			Err((sess, err)) => Err((IAm(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IAm<Session>> {
		Ok(IAm(self.0.duplicate()?))
	}
}

impl<T> Deref for IAm<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IAm<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IAm<T> {
	pub fn unknown0(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown1(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown2(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IAm<T> {
	fn from(obj: T) -> IAm<T> {
		IAm(obj)
	}
}
