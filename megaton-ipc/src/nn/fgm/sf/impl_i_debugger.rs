
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IDebugger<T>(T);

impl IDebugger<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IDebugger<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IDebugger(domain)),
			Err((sess, err)) => Err((IDebugger(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IDebugger<Session>> {
		Ok(IDebugger(self.0.duplicate()?))
	}
}

impl<T> Deref for IDebugger<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IDebugger<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IDebugger<T> {
	pub fn initialize(&self, unk0: u64, unk1: &KObject) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 1], [_; 0]> = Request::new(0)
			.args(unk0)
			.copy_handle(unk1)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn read(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn cancel(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IDebugger<T> {
	fn from(obj: T) -> IDebugger<T> {
		IDebugger(obj)
	}
}
