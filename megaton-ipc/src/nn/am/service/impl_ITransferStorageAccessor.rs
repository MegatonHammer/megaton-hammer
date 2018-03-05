
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct ITransferStorageAccessor(Session);

impl ITransferStorageAccessor {
	pub fn GetSize(&self, ) -> Result<i64> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	// fn GetHandle(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for ITransferStorageAccessor {
	unsafe fn from_kobject(obj: KObject) -> ITransferStorageAccessor {
		ITransferStorageAccessor(Session::from_kobject(obj))
	}
}
