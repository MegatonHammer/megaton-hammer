
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IDebugManager(Session);

impl IDebugManager {
	pub fn get_service() -> Result<IDebugManager> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"nfp:dbg\0").map(|s| unsafe { IDebugManager::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IDebugManager {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IDebugManager {
	pub fn Unknown0(&self, ) -> Result<Session> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IDebugManager {
	unsafe fn from_kobject(obj: KObject) -> IDebugManager {
		IDebugManager(Session::from_kobject(obj))
	}
}
