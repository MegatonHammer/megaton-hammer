
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IStaticService(Session);

impl IStaticService {
	pub fn get_service() -> Result<IStaticService> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"nifm:a\0\0").map(|s| unsafe { IStaticService::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		let r = sm.GetService(*b"nifm:s\0\0").map(|s| unsafe { IStaticService::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		let r = sm.GetService(*b"nifm:u\0\0").map(|s| unsafe { IStaticService::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IStaticService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IStaticService {
	pub fn CreateGeneralServiceOld(&self, ) -> Result<::nn::nifm::detail::IGeneralService> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn CreateGeneralService(&self, unk0: u64) -> Result<::nn::nifm::detail::IGeneralService> {
		let req = Request::new(5)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IStaticService {
	unsafe fn from_kobject(obj: KObject) -> IStaticService {
		IStaticService(Session::from_kobject(obj))
	}
}
