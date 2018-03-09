
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IBoardPowerControlManager(Session);

impl IBoardPowerControlManager {
	pub fn get_service() -> Result<IBoardPowerControlManager> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"bpc\0\0\0\0\0").map(|s| unsafe { IBoardPowerControlManager::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IBoardPowerControlManager {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IBoardPowerControlManager {
	pub fn ShutdownSystem(&self, ) -> Result<()> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn RebootSystem(&self, ) -> Result<()> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetWakeupReason(&self, ) -> Result<u32> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetShutdownReason(&self, ) -> Result<u32> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetAcOk(&self, ) -> Result<u8> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetBoardPowerControlEvent(&self, unk0: u32) -> Result<KObject> {
		let req = Request::new(5)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn GetSleepButtonState(&self, ) -> Result<u32> {
		let req = Request::new(6)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetPowerEvent(&self, unk0: u32) -> Result<KObject> {
		let req = Request::new(7)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn Unknown8(&self, unk0: u64) -> Result<u32> {
		let req = Request::new(8)
			.args(unk0)
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn Unknown9(&self, unk0: u32) -> Result<()> {
		let req = Request::new(9)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown10(&self, ) -> Result<()> {
		let req = Request::new(10)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IBoardPowerControlManager {
	unsafe fn from_kobject(obj: KObject) -> IBoardPowerControlManager {
		IBoardPowerControlManager(Session::from_kobject(obj))
	}
}
