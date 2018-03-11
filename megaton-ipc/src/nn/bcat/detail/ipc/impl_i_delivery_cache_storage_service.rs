
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IDeliveryCacheStorageService(Session);

impl AsRef<Session> for IDeliveryCacheStorageService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IDeliveryCacheStorageService {
	pub fn create_file_service(&self, ) -> Result<::nn::bcat::detail::ipc::IDeliveryCacheFileService> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn create_directory_service(&self, ) -> Result<::nn::bcat::detail::ipc::IDeliveryCacheDirectoryService> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	// fn enumerate_delivery_cache_directory(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IDeliveryCacheStorageService {
	unsafe fn from_kobject(obj: KObject) -> IDeliveryCacheStorageService {
		IDeliveryCacheStorageService(Session::from_kobject(obj))
	}
}
