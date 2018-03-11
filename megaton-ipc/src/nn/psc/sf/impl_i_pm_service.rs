
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IPmService(Session);

impl IPmService {
	pub fn new() -> Result<IPmService> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"psc:m\0\0\0").map(|s| unsafe { IPmService::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IPmService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IPmService {
	pub fn get_i_pm_module(&self, ) -> Result<::nn::psc::sf::IPmModule> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IPmService {
	unsafe fn from_kobject(obj: KObject) -> IPmService {
		IPmService(Session::from_kobject(obj))
	}
}
