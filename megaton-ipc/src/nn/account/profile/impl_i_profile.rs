
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IProfile<T>(T);

impl IProfile<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IProfile<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IProfile(domain)),
			Err((sess, err)) => Err((IProfile(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IProfile<Session>> {
		Ok(IProfile(self.0.duplicate()?))
	}
}

impl<T> Deref for IProfile<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IProfile<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IProfile<T> {
	pub fn get(&self, unk1: &mut ::nn::account::profile::UserData) -> Result<::nn::account::profile::ProfileBase> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk1, 0x1a))
			;
		let res : Response<::nn::account::profile::ProfileBase> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_base(&self, ) -> Result<::nn::account::profile::ProfileBase> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let res : Response<::nn::account::profile::ProfileBase> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_image_size(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn load_image(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IProfile<T> {
	fn from(obj: T) -> IProfile<T> {
		IProfile(obj)
	}
}
