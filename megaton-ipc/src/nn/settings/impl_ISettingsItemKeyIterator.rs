
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct ISettingsItemKeyIterator(Session);

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
