
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IServiceCreator(Session);

impl IServiceCreator {
	pub fn new() -> Result<IServiceCreator> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"bcat:a\0\0").map(|s| unsafe { IServiceCreator::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		let r = sm.get_service(*b"bcat:m\0\0").map(|s| unsafe { IServiceCreator::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		let r = sm.get_service(*b"bcat:u\0\0").map(|s| unsafe { IServiceCreator::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		let r = sm.get_service(*b"bcat:s\0\0").map(|s| unsafe { IServiceCreator::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IServiceCreator {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IServiceCreator {
	pub fn create_bcat_service(&self, unk0: u64) -> Result<::nn::bcat::detail::ipc::IBcatService> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn create_delivery_cache_storage_service(&self, unk0: u64) -> Result<::nn::bcat::detail::ipc::IDeliveryCacheStorageService> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn create_delivery_cache_storage_service_with_application_id(&self, unk0: ::nn::ApplicationId) -> Result<::nn::bcat::detail::ipc::IDeliveryCacheStorageService> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IServiceCreator {
	unsafe fn from_kobject(obj: KObject) -> IServiceCreator {
		IServiceCreator(Session::from_kobject(obj))
	}
}
