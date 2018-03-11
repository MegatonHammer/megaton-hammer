
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hammer::ipc::IPCBuffer;

#[derive(Debug)]
pub struct IDirectory(Session);

impl AsRef<Session> for IDirectory {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IDirectory {
	// fn Read(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetEntryCount(&self, ) -> Result<u64> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IDirectory {
	unsafe fn from_kobject(obj: KObject) -> IDirectory {
		IDirectory(Session::from_kobject(obj))
	}
}
