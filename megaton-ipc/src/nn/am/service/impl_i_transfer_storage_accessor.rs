
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct ITransferStorageAccessor(Session);

impl AsRef<Session> for ITransferStorageAccessor {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ITransferStorageAccessor {
	pub fn get_size(&self, ) -> Result<i64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_handle(&self, ) -> Result<(u64, KObject)> {
		use megaton_hammer::ipc::{Request, Response};

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
