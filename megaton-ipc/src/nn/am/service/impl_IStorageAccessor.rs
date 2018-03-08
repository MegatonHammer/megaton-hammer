
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IStorageAccessor(Session);

impl IStorageAccessor {
	pub fn GetSize(&self, ) -> Result<i64> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn Write(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Read(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IStorageAccessor {
	unsafe fn from_kobject(obj: KObject) -> IStorageAccessor {
		IStorageAccessor(Session::from_kobject(obj))
	}
}
