
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IStorage(Session);

impl AsRef<Session> for IStorage {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IStorage {
	pub fn Open(&self, ) -> Result<::nn::am::service::IStorageAccessor> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn OpenTransferStorage(&self, ) -> Result<::nn::am::service::ITransferStorageAccessor> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IStorage {
	unsafe fn from_kobject(obj: KObject) -> IStorage {
		IStorage(Session::from_kobject(obj))
	}
}
