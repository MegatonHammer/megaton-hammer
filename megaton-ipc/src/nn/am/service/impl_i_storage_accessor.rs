
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IStorageAccessor(Session);

impl AsRef<Session> for IStorageAccessor {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IStorageAccessor {
	pub fn get_size(&self, ) -> Result<i64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn write(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn read(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IStorageAccessor {
	unsafe fn from_kobject(obj: KObject) -> IStorageAccessor {
		IStorageAccessor(Session::from_kobject(obj))
	}
}
