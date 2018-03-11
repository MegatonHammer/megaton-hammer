
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hamer::ipc::IPCBuffer

#[derive(Debug)]
pub struct IFileSystem(Session);

impl AsRef<Session> for IFileSystem {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IFileSystem {
	// fn CreateFile(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn DeleteFile(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn CreateDirectory(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn DeleteDirectory(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn DeleteDirectoryRecursively(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn RenameFile(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn RenameDirectory(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetEntryType(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn OpenFile(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn OpenDirectory(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Commit(&self, ) -> Result<()> {
		let req = Request::new(10)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn GetFreeSpaceSize(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetTotalSpaceSize(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn CleanDirectoryRecursively(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetFileTimeStampRaw(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IFileSystem {
	unsafe fn from_kobject(obj: KObject) -> IFileSystem {
		IFileSystem(Session::from_kobject(obj))
	}
}
