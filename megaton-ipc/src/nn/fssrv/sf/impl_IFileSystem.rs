
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IFileSystem(Session);

impl IFileSystem {
	pub fn CreateFile(&self, mode: u32, size: u64, path: &[u8; 0x301]) -> Result<()> {
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
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DeleteFile(&self, path: &[u8; 0x301]) -> Result<()> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn CreateDirectory(&self, path: &[u8; 0x301]) -> Result<()> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DeleteDirectory(&self, path: &[u8; 0x301]) -> Result<()> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DeleteDirectoryRecursively(&self, path: &[u8; 0x301]) -> Result<()> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn RenameFile(&self, oldPath: &[u8; 0x301], newPath: &[u8; 0x301]) -> Result<()> {
		let req = Request::new(5)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn RenameDirectory(&self, oldPath: &[u8; 0x301], newPath: &[u8; 0x301]) -> Result<()> {
		let req = Request::new(6)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetEntryType(&self, path: &[u8; 0x301]) -> Result<(u32)> {
		let req = Request::new(7)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn OpenFile(&self, mode: u32, path: &[u8; 0x301]) -> Result<(::nn::fssrv::sf::IFile)> {
		let req = Request::new(8)
			.args(mode)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn OpenDirectory(&self, unk0: u32, path: &[u8; 0x301]) -> Result<(::nn::fssrv::sf::IDirectory)> {
		let req = Request::new(9)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn Commit(&self, ) -> Result<()> {
		let req = Request::new(10)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetFreeSpaceSize(&self, path: &[u8; 0x301]) -> Result<(u64)> {
		let req = Request::new(11)
			.args(())
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetTotalSpaceSize(&self, path: &[u8; 0x301]) -> Result<(u64)> {
		let req = Request::new(12)
			.args(())
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn CleanDirectoryRecursively(&self, path: &[u8; 0x301]) -> Result<()> {
		let req = Request::new(13)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn GetFileTimeStampRaw(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IFileSystem {
	unsafe fn from_kobject(obj: KObject) -> IFileSystem {
		IFileSystem(Session::from_kobject(obj))
	}
}
