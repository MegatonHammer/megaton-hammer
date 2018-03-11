
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IDebugMonitorInterface(Session);

impl IDebugMonitorInterface {
	pub fn new() -> Result<IDebugMonitorInterface> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"ro:dmnt\0").map(|s| unsafe { IDebugMonitorInterface::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IDebugMonitorInterface {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IDebugMonitorInterface {
	pub fn unknown0(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IDebugMonitorInterface {
	unsafe fn from_kobject(obj: KObject) -> IDebugMonitorInterface {
		IDebugMonitorInterface(Session::from_kobject(obj))
	}
}
