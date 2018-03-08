
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct INvDrvDebugFSServices(Session);

impl INvDrvDebugFSServices {
	pub fn get_service() -> Result<INvDrvDebugFSServices> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"nvdrvdbg").map(|s| unsafe { INvDrvDebugFSServices::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl INvDrvDebugFSServices {
	pub fn OpenLog(&self, ) -> Result<()> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn CloseLog(&self, ) -> Result<()> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ReadLog(&self, ) -> Result<()> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for INvDrvDebugFSServices {
	unsafe fn from_kobject(obj: KObject) -> INvDrvDebugFSServices {
		INvDrvDebugFSServices(Session::from_kobject(obj))
	}
}
