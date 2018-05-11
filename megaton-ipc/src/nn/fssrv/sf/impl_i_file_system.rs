
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IFileSystem<T>(T);

impl IFileSystem<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IFileSystem<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IFileSystem(domain)),
			Err((sess, err)) => Err((IFileSystem(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IFileSystem<Session>> {
		Ok(IFileSystem(self.0.duplicate()?))
	}
}

impl<T> Deref for IFileSystem<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IFileSystem<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IFileSystem<T> {
	pub fn create_file(&self, mode: u32, size: u64, path: &[u8; 0x301]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			mode: u32,
			size: u64,
		}
		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(0)
			.args(InRaw {
				mode,
				size,
			})
			.descriptor(IPCBuffer::from_ref(path, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn delete_file(&self, path: &[u8; 0x301]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			.descriptor(IPCBuffer::from_ref(path, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn create_directory(&self, path: &[u8; 0x301]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			.descriptor(IPCBuffer::from_ref(path, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn delete_directory(&self, path: &[u8; 0x301]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(3)
			.args(())
			.descriptor(IPCBuffer::from_ref(path, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn delete_directory_recursively(&self, path: &[u8; 0x301]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(4)
			.args(())
			.descriptor(IPCBuffer::from_ref(path, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn rename_file(&self, old_path: &[u8; 0x301], new_path: &[u8; 0x301]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 2], [_; 0], [_; 0]> = Request::new(5)
			.args(())
			.descriptor(IPCBuffer::from_ref(old_path, 0x19))
			.descriptor(IPCBuffer::from_ref(new_path, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn rename_directory(&self, old_path: &[u8; 0x301], new_path: &[u8; 0x301]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 2], [_; 0], [_; 0]> = Request::new(6)
			.args(())
			.descriptor(IPCBuffer::from_ref(old_path, 0x19))
			.descriptor(IPCBuffer::from_ref(new_path, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_entry_type(&self, path: &[u8; 0x301]) -> Result<u32> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(7)
			.args(())
			.descriptor(IPCBuffer::from_ref(path, 0x19))
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn open_file(&self, mode: u32, path: &[u8; 0x301]) -> Result<::nn::fssrv::sf::IFile<T>> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(8)
			.args(mode)
			.descriptor(IPCBuffer::from_ref(path, 0x19))
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn open_directory(&self, unk0: u32, path: &[u8; 0x301]) -> Result<::nn::fssrv::sf::IDirectory<T>> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(9)
			.args(unk0)
			.descriptor(IPCBuffer::from_ref(path, 0x19))
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn commit(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(10)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_free_space_size(&self, path: &[u8; 0x301]) -> Result<u64> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(11)
			.args(())
			.descriptor(IPCBuffer::from_ref(path, 0x19))
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_total_space_size(&self, path: &[u8; 0x301]) -> Result<u64> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(12)
			.args(())
			.descriptor(IPCBuffer::from_ref(path, 0x19))
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn clean_directory_recursively(&self, path: &[u8; 0x301]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(13)
			.args(())
			.descriptor(IPCBuffer::from_ref(path, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_file_time_stamp_raw(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IFileSystem<T> {
	fn from(obj: T) -> IFileSystem<T> {
		IFileSystem(obj)
	}
}
