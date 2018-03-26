
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct INewsDataService<T>(T);

impl INewsDataService<Session> {
	pub fn to_domain(self) -> ::core::result::Result<INewsDataService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(INewsDataService(domain)),
			Err((sess, err)) => Err((INewsDataService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<INewsDataService<Session>> {
		Ok(INewsDataService(self.0.duplicate()?))
	}
}

impl<T> Deref for INewsDataService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for INewsDataService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> INewsDataService<T> {
	// fn unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown3(&self, ) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl<T: Object> From<T> for INewsDataService<T> {
	fn from(obj: T) -> INewsDataService<T> {
		INewsDataService(obj)
	}
}
