
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct ISettingsItemKeyIterator(Session);

impl AsRef<Session> for ISettingsItemKeyIterator {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ISettingsItemKeyIterator {
	pub fn go_next(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_key_size(&self, ) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_key(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for ISettingsItemKeyIterator {
	unsafe fn from_kobject(obj: KObject) -> ISettingsItemKeyIterator {
		ISettingsItemKeyIterator(Session::from_kobject(obj))
	}
}
