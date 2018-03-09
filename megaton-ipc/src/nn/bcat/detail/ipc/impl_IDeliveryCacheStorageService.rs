
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IDeliveryCacheStorageService(Session);

impl AsRef<Session> for IDeliveryCacheStorageService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IDeliveryCacheStorageService {
	pub fn CreateFileService(&self, ) -> Result<::nn::bcat::detail::ipc::IDeliveryCacheFileService> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn CreateDirectoryService(&self, ) -> Result<::nn::bcat::detail::ipc::IDeliveryCacheDirectoryService> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	// fn EnumerateDeliveryCacheDirectory(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IDeliveryCacheStorageService {
	unsafe fn from_kobject(obj: KObject) -> IDeliveryCacheStorageService {
		IDeliveryCacheStorageService(Session::from_kobject(obj))
	}
}
