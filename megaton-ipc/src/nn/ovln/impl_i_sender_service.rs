
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct ISenderService(Session);

impl ISenderService {
	pub fn new() -> Result<ISenderService> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"ovln:snd").map(|s| unsafe { ISenderService::from_kobject(s) });
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
	pub fn unknown0(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for ISenderService {
	unsafe fn from_kobject(obj: KObject) -> ISenderService {
		ISenderService(Session::from_kobject(obj))
	}
}
