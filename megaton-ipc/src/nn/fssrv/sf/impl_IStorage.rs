
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hammer::ipc::IPCBuffer;

#[derive(Debug)]
pub struct IStorage(Session);

impl AsRef<Session> for IStorage {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IStorage {
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

impl FromKObject for IStorage {
	unsafe fn from_kobject(obj: KObject) -> IStorage {
		IStorage(Session::from_kobject(obj))
	}
}
