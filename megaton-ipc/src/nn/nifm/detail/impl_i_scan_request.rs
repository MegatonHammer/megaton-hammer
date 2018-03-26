
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IScanRequest<T>(T);

impl IScanRequest<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IScanRequest<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IScanRequest(domain)),
			Err((sess, err)) => Err((IScanRequest(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IScanRequest<Session>> {
		Ok(IScanRequest(self.0.duplicate()?))
	}
}

impl<T> Deref for IScanRequest<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IScanRequest<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IScanRequest<T> {
	pub fn submit(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_processing(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_result(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_system_event_readable_handle(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

}

impl<T: Object> From<T> for IScanRequest<T> {
	fn from(obj: T) -> IScanRequest<T> {
		IScanRequest(obj)
	}
}
