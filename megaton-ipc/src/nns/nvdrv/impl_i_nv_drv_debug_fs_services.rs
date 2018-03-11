
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct INvDrvDebugFSServices(Session);

impl INvDrvDebugFSServices {
	pub fn new() -> Result<INvDrvDebugFSServices> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"nvdrvdbg").map(|s| unsafe { INvDrvDebugFSServices::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for INvDrvDebugFSServices {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl INvDrvDebugFSServices {
	pub fn open_log(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn close_log(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn read_log(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for INvDrvDebugFSServices {
	unsafe fn from_kobject(obj: KObject) -> INvDrvDebugFSServices {
		INvDrvDebugFSServices(Session::from_kobject(obj))
	}
}
