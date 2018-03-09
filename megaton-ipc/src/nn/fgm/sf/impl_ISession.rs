
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct ISession(Session);

impl ISession {
	pub fn get_service() -> Result<ISession> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"fgm:0\0\0\0").map(|s| unsafe { ISession::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		let r = sm.GetService(*b"fgm\0\0\0\0\0").map(|s| unsafe { ISession::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		let r = sm.GetService(*b"fgm:9\0\0\0").map(|s| unsafe { ISession::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for ISession {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ISession {
	pub fn Initialize(&self, ) -> Result<::nn::fgm::sf::IRequest> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for ISession {
	unsafe fn from_kobject(obj: KObject) -> ISession {
		ISession(Session::from_kobject(obj))
	}
}
