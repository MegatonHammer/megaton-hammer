
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hamer::ipc::IPCBuffer

#[derive(Debug)]
pub struct ISettingsItemKeyIterator(Session);

impl AsRef<Session> for ISettingsItemKeyIterator {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ISettingsItemKeyIterator {
	pub fn GoNext(&self, ) -> Result<()> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetKeySize(&self, ) -> Result<u64> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn GetKey(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for ISettingsItemKeyIterator {
	unsafe fn from_kobject(obj: KObject) -> ISettingsItemKeyIterator {
		ISettingsItemKeyIterator(Session::from_kobject(obj))
	}
}
