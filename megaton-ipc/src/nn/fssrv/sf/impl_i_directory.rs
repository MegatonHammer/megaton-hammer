
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IDirectory<T>(T);

impl IDirectory<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IDirectory<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IDirectory(domain)),
			Err((sess, err)) => Err((IDirectory(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IDirectory<Session>> {
		Ok(IDirectory(self.0.duplicate()?))
	}
}

impl<T> Deref for IDirectory<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IDirectory<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IDirectory<T> {
	pub fn read(&self, unk1: &mut [::nn::fssrv::sf::IDirectoryEntry]) -> Result<u64> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			.descriptor(IPCBuffer::from_mut_slice(unk1, 6))
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_entry_count(&self, ) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl<T: Object> From<T> for IDirectory<T> {
	fn from(obj: T) -> IDirectory<T> {
		IDirectory(obj)
	}
}
