
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IBootModeInterface(Session);

impl IBootModeInterface {
	pub fn new() -> Result<IBootModeInterface> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"pm:bm\0\0\0").map(|s| unsafe { IBootModeInterface::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IBootModeInterface {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IBootModeInterface {
	pub fn get_boot_mode(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_maintenance_boot(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IBootModeInterface {
	unsafe fn from_kobject(obj: KObject) -> IBootModeInterface {
		IBootModeInterface(Session::from_kobject(obj))
	}
}
