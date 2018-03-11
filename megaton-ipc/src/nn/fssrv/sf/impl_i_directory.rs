
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IDirectory(Session);

impl AsRef<Session> for IDirectory {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IDirectory {
	// fn read(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_entry_count(&self, ) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IDirectory {
	unsafe fn from_kobject(obj: KObject) -> IDirectory {
		IDirectory(Session::from_kobject(obj))
	}
}
