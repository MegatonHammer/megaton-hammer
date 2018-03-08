
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
		let r = sm.GetService(*b"friend:v").map(|s| unsafe { IServiceCreator::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		let r = sm.GetService(*b"friend:u").map(|s| unsafe { IServiceCreator::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		let r = sm.GetService(*b"friend:m").map(|s| unsafe { IServiceCreator::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		let r = sm.GetService(*b"friend:s").map(|s| unsafe { IServiceCreator::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		let r = sm.GetService(*b"friend:a").map(|s| unsafe { IServiceCreator::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl IServiceCreator {
	pub fn CreateFriendService(&self, ) -> Result<::nn::friends::detail::ipc::IFriendService> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn CreateNotificationService(&self, unk0: ::nn::account::Uid) -> Result<::nn::friends::detail::ipc::INotificationService> {
		let req = Request::new(1)
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
