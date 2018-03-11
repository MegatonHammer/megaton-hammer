
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IManagerPrivileged(Session);

impl IManagerPrivileged {
	pub fn new() -> Result<IManagerPrivileged> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"apm:p\0\0\0").map(|s| unsafe { IManagerPrivileged::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IManagerPrivileged {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IManagerPrivileged {
	pub fn open_session(&self, ) -> Result<::nn::apm::ISession> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IManagerPrivileged {
	unsafe fn from_kobject(obj: KObject) -> IManagerPrivileged {
		IManagerPrivileged(Session::from_kobject(obj))
	}
}
