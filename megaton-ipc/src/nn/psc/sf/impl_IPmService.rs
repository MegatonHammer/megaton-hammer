
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IPmService(Session);

impl IPmService {
	pub fn get_service() -> Result<IPmService> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"psc:m\0\0\0").map(|s| unsafe { IPmService::from_kobject(s) });
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
	pub fn GetIPmModule(&self, ) -> Result<::nn::psc::sf::IPmModule> {
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
