
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IRtcManager(Session);

impl IRtcManager {
	pub fn get_service() -> Result<IRtcManager> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"bpc:r\0\0\0").map(|s| unsafe { IRtcManager::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IRtcManager {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IRtcManager {
	pub fn GetExternalRtcValue(&self, ) -> Result<u64> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SetExternalRtcValue(&self, unk0: u64) -> Result<()> {
		let req = Request::new(1)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ReadExternalRtcResetFlag(&self, ) -> Result<u8> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn ClearExternalRtcResetFlag(&self, ) -> Result<()> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IRtcManager {
	unsafe fn from_kobject(obj: KObject) -> IRtcManager {
		IRtcManager(Session::from_kobject(obj))
	}
}
