
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IJitEnvironment<T>(T);

impl IJitEnvironment<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IJitEnvironment<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IJitEnvironment(domain)),
			Err((sess, err)) => Err((IJitEnvironment(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IJitEnvironment<Session>> {
		Ok(IJitEnvironment(self.0.duplicate()?))
	}
}

impl<T> Deref for IJitEnvironment<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IJitEnvironment<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IJitEnvironment<T> {
	pub fn control(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn generate_code(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn load_plugin(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1000)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_code_address(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1001)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IJitEnvironment<T> {
	fn from(obj: T) -> IJitEnvironment<T> {
		IJitEnvironment(obj)
	}
}
