
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IServiceManager(Session);

impl IServiceManager {
	pub fn new() -> Result<IServiceManager> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"htc:tenv").map(|s| unsafe { IServiceManager::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IServiceManager {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IServiceManager {
	pub fn open_service(&self, unk0: u64) -> Result<Session> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IServiceManager {
	unsafe fn from_kobject(obj: KObject) -> IServiceManager {
		IServiceManager(Session::from_kobject(obj))
	}
}
