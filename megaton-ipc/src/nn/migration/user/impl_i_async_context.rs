
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IAsyncContext<T>(T);

impl IAsyncContext<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IAsyncContext<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IAsyncContext(domain)),
			Err((sess, err)) => Err((IAsyncContext(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IAsyncContext<Session>> {
		Ok(IAsyncContext(self.0.duplicate()?))
	}
}

impl<T> Deref for IAsyncContext<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IAsyncContext<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IAsyncContext<T> {
	pub fn get_system_event(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn cancel(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn has_done(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_result(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IAsyncContext<T> {
	fn from(obj: T) -> IAsyncContext<T> {
		IAsyncContext(obj)
	}
}
