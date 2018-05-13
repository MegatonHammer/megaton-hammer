
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct ISettingsItemKeyIterator<T>(T);

impl ISettingsItemKeyIterator<Session> {
	pub fn to_domain(self) -> ::core::result::Result<ISettingsItemKeyIterator<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ISettingsItemKeyIterator(domain)),
			Err((sess, err)) => Err((ISettingsItemKeyIterator(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ISettingsItemKeyIterator<Session>> {
		Ok(ISettingsItemKeyIterator(self.0.duplicate()?))
	}
}

impl<T> Deref for ISettingsItemKeyIterator<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ISettingsItemKeyIterator<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ISettingsItemKeyIterator<T> {
	pub fn go_next(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_key_size(&self, ) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_key(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for ISettingsItemKeyIterator<T> {
	fn from(obj: T) -> ISettingsItemKeyIterator<T> {
		ISettingsItemKeyIterator(obj)
	}
}
