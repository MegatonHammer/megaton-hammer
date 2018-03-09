
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct ISenderService(Session);

impl ISenderService {
	pub fn get_service() -> Result<ISenderService> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"ovln:snd").map(|s| unsafe { ISenderService::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for ISenderService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ISenderService {
	pub fn Unknown0(&self, ) -> Result<()> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for ISenderService {
	unsafe fn from_kobject(obj: KObject) -> ISenderService {
		ISenderService(Session::from_kobject(obj))
	}
}
