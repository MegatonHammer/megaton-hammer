
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IFileSystem(Session);

impl AsRef<Session> for IFileSystem {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IFileSystem {
	pub fn create_file(&self, mode: u32, size: u64, path: &[u8; 0x301]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			mode: u32,
			size: u64,
		}
		let req = Request::new(0)
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

		let req = Request::new(1)
			.args(())
			.descriptor(IPCBuffer::from_ref(path, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn create_directory(&self, path: &[u8; 0x301]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			.descriptor(IPCBuffer::from_ref(path, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn delete_directory(&self, path: &[u8; 0x301]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			.descriptor(IPCBuffer::from_ref(path, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn delete_directory_recursively(&self, path: &[u8; 0x301]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(())
			.descriptor(IPCBuffer::from_ref(path, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn rename_file(&self, old_path: &[u8; 0x301], new_path: &[u8; 0x301]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(5)
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

		let req = Request::new(6)
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

		let req = Request::new(7)
			.args(())
			.descriptor(IPCBuffer::from_ref(path, 0x19))
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn open_file(&self, mode: u32, path: &[u8; 0x301]) -> Result<::nn::fssrv::sf::IFile> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(8)
			.args(mode)
			.descriptor(IPCBuffer::from_ref(path, 0x19))
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn open_directory(&self, unk0: u32, path: &[u8; 0x301]) -> Result<::nn::fssrv::sf::IDirectory> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(9)
			.args(unk0)
			.descriptor(IPCBuffer::from_ref(path, 0x19))
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn commit(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_free_space_size(&self, path: &[u8; 0x301]) -> Result<u64> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(11)
			.args(())
			.descriptor(IPCBuffer::from_ref(path, 0x19))
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_total_space_size(&self, path: &[u8; 0x301]) -> Result<u64> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(12)
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

		let req = Request::new(13)
			.args(())
			.descriptor(IPCBuffer::from_ref(path, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_file_time_stamp_raw(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IFileSystem {
	unsafe fn from_kobject(obj: KObject) -> IFileSystem {
		IFileSystem(Session::from_kobject(obj))
	}
}
