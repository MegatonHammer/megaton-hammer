
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IProfileEditor<T>(T);

impl IProfileEditor<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IProfileEditor<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IProfileEditor(domain)),
			Err((sess, err)) => Err((IProfileEditor(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IProfileEditor<Session>> {
		Ok(IProfileEditor(self.0.duplicate()?))
	}
}

impl<T> Deref for IProfileEditor<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IProfileEditor<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IProfileEditor<T> {
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
	pub fn store(&self, unk0: ::nn::account::profile::ProfileBase, unk1: &::nn::account::profile::UserData) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(100)
			.args(unk0)
			.descriptor(IPCBuffer::from_ref(unk1, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn store_with_image(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IProfileEditor<T> {
	fn from(obj: T) -> IProfileEditor<T> {
		IProfileEditor(obj)
	}
}
