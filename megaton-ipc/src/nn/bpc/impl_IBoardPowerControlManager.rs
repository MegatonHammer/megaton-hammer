
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IBoardPowerControlManager(Session);

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
	// fn GetBoardPowerControlEvent(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetSleepButtonState(&self, ) -> Result<u32> {
		let req = Request::new(6)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	// fn GetPowerEvent(&self, UNKNOWN) -> Result<UNKNOWN>;
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
