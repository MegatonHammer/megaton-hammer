
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IAmManager(Session);

impl IAmManager {
	pub fn new() -> Result<IAmManager> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"nfc:am\0\0").map(|s| unsafe { IAmManager::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IAmManager {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IAmManager {
	pub fn unknown0(&self, ) -> Result<Session> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IAmManager {
	unsafe fn from_kobject(obj: KObject) -> IAmManager {
		IAmManager(Session::from_kobject(obj))
	}
}
