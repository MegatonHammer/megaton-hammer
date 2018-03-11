
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IUserManager(Session);

impl IUserManager {
	pub fn new() -> Result<IUserManager> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"nfp:user").map(|s| unsafe { IUserManager::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IUserManager {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IUserManager {
	pub fn unknown0(&self, ) -> Result<Session> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IUserManager {
	unsafe fn from_kobject(obj: KObject) -> IUserManager {
		IUserManager(Session::from_kobject(obj))
	}
}
