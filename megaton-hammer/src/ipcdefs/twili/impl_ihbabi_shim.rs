
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IHBABIShim<T>(T);

impl IHBABIShim<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IHBABIShim<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IHBABIShim(domain)),
			Err((sess, err)) => Err((IHBABIShim(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IHBABIShim<Session>> {
		Ok(IHBABIShim(self.0.duplicate()?))
	}
}

impl<T> Deref for IHBABIShim<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IHBABIShim<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IHBABIShim<T> {
	pub fn get_process_handle(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn get_loader_config_entry_count(&self, ) -> Result<u32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_loader_config_entries(&self, unk0: &mut [::ipcdefs::LoaderConfigEntryT]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			.descriptor(IPCBuffer::from_mut_slice(unk0, 6))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_loader_config_handle(&self, placeholder: u32) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(3)
			.args(placeholder)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn set_next_load_path(&self, path: &[u8], argv: &[u8]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 2], [_; 0], [_; 0]> = Request::new(4)
			.args(())
			.descriptor(IPCBuffer::from_slice(path, 5))
			.descriptor(IPCBuffer::from_slice(argv, 5))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_target_entry_point(&self, ) -> Result<u64> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(5)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_exit_code(&self, unk0: u32) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(6)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IHBABIShim<T> {
	fn from(obj: T) -> IHBABIShim<T> {
		IHBABIShim(obj)
	}
}
