
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hammer::ipc::IPCBuffer;

#[derive(Debug)]
pub struct IFile(Session);

impl AsRef<Session> for IFile {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IFile {
	// fn Read(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Write(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Flush(&self, ) -> Result<()> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetSize(&self, size: u64) -> Result<()> {
		let req = Request::new(3)
			.args(size)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetSize(&self, ) -> Result<u64> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IFile {
	unsafe fn from_kobject(obj: KObject) -> IFile {
		IFile(Session::from_kobject(obj))
	}
}
