
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct ISession<T>(T);

impl ISession<Session> {
	pub fn to_domain(self) -> ::core::result::Result<ISession<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ISession(domain)),
			Err((sess, err)) => Err((ISession(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ISession<Session>> {
		Ok(ISession(self.0.duplicate()?))
	}
}

impl<T> Deref for ISession<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ISession<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ISession<T> {
	// fn unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn send(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn receive(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn execute_command_list(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(5)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for ISession<T> {
	fn from(obj: T) -> ISession<T> {
		ISession(obj)
	}
}
