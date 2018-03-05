
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct ISystemClock(Session);

impl ISystemClock {
	pub fn GetCurrentTime(&self, ) -> Result<::nn::time::PosixTime> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<::nn::time::PosixTime> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetCurrentTime(&self, unk0: ::nn::time::PosixTime) -> Result<()> {
		let req = Request::new(1)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetSystemClockContext(&self, ) -> Result<::nn::time::SystemClockContext> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<::nn::time::SystemClockContext> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetSystemClockContext(&self, unk0: ::nn::time::SystemClockContext) -> Result<()> {
		let req = Request::new(3)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
}

impl FromKObject for ISystemClock {
	unsafe fn from_kobject(obj: KObject) -> ISystemClock {
		ISystemClock(Session::from_kobject(obj))
	}
}
