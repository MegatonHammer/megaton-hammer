
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct ITransferStorageAccessor(Session);

impl ITransferStorageAccessor {
	pub fn GetSize(&self, ) -> Result<i64> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetHandle(&self, ) -> Result<(u64, KObject)> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok((*res.get_raw(),res.pop_handle()))
	}

}

impl FromKObject for ITransferStorageAccessor {
	unsafe fn from_kobject(obj: KObject) -> ITransferStorageAccessor {
		ITransferStorageAccessor(Session::from_kobject(obj))
	}
}
