
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IServiceCreator(Session);

impl IServiceCreator {
	pub fn get_service() -> Result<IServiceCreator> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"bcat:a\0\0").map(|s| unsafe { IServiceCreator::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		let r = sm.GetService(*b"bcat:m\0\0").map(|s| unsafe { IServiceCreator::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		let r = sm.GetService(*b"bcat:u\0\0").map(|s| unsafe { IServiceCreator::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		let r = sm.GetService(*b"bcat:s\0\0").map(|s| unsafe { IServiceCreator::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl IServiceCreator {
	pub fn CreateBcatService(&self, unk0: u64) -> Result<::nn::bcat::detail::ipc::IBcatService> {
		let req = Request::new(0)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn CreateDeliveryCacheStorageService(&self, unk0: u64) -> Result<::nn::bcat::detail::ipc::IDeliveryCacheStorageService> {
		let req = Request::new(1)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn CreateDeliveryCacheStorageServiceWithApplicationId(&self, unk0: ::nn::ApplicationId) -> Result<::nn::bcat::detail::ipc::IDeliveryCacheStorageService> {
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
