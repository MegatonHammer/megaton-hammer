
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IBtmSystem(Session);

impl IBtmSystem {
	pub fn new() -> Result<IBtmSystem> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"btm:sys\0").map(|s| unsafe { IBtmSystem::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IBtmSystem {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IBtmSystem {
	pub fn unknown0(&self, ) -> Result<Session> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IBtmSystem {
	unsafe fn from_kobject(obj: KObject) -> IBtmSystem {
		IBtmSystem(Session::from_kobject(obj))
	}
}
