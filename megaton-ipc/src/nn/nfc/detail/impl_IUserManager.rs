
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IUserManager(Session);

impl IUserManager {
	pub fn get_service() -> Result<IUserManager> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"nfc:user").map(|s| unsafe { IUserManager::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl IUserManager {
	pub fn Unknown0(&self, ) -> Result<Session> {
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
