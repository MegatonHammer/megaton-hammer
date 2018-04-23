
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct ISystemClock<T>(T);

impl ISystemClock<Session> {
	pub fn to_domain(self) -> ::core::result::Result<ISystemClock<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ISystemClock(domain)),
			Err((sess, err)) => Err((ISystemClock(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ISystemClock<Session>> {
		Ok(ISystemClock(self.0.duplicate()?))
	}
}

impl<T> Deref for ISystemClock<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ISystemClock<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ISystemClock<T> {
	pub fn get_current_time(&self, ) -> Result<::nn::time::PosixTime> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let res : Response<::nn::time::PosixTime> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_current_time(&self, unk0: ::nn::time::PosixTime) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_system_clock_context(&self, ) -> Result<::nn::time::SystemClockContext> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let res : Response<::nn::time::SystemClockContext> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_system_clock_context(&self, unk0: ::nn::time::SystemClockContext) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for ISystemClock<T> {
	fn from(obj: T) -> ISystemClock<T> {
		ISystemClock(obj)
	}
}
