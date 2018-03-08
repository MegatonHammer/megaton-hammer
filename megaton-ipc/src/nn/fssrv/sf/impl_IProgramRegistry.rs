
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IProgramRegistry(Session);

impl IProgramRegistry {
	pub fn get_service() -> Result<IProgramRegistry> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"fsp-pr\0\0").map(|s| unsafe { IProgramRegistry::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl IProgramRegistry {
	// fn SetFsPermissions(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn ClearFsPermissions(&self, pid: u64) -> Result<()> {
		let req = Request::new(1)
			.args(pid)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetEnabledProgramVerification(&self, enabled: u8) -> Result<()> {
		let req = Request::new(256)
			.args(enabled)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IProgramRegistry {
	unsafe fn from_kobject(obj: KObject) -> IProgramRegistry {
		IProgramRegistry(Session::from_kobject(obj))
	}
}
