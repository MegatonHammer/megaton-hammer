
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct ISteadyClock(Session);

impl ISteadyClock {
	pub fn GetCurrentTimePoint(&self, ) -> Result<(::nn::time::SteadyClockTimePoint)> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<::nn::time::SteadyClockTimePoint> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetTestOffset(&self, ) -> Result<(::nn::TimeSpanType)> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<::nn::TimeSpanType> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SetTestOffset(&self, unk0: ::nn::TimeSpanType) -> Result<()> {
		let req = Request::new(3)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetRtcValue(&self, ) -> Result<(i64)> {
		let req = Request::new(100)
			.args(())
			;
		let mut res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn IsRtcResetDetected(&self, ) -> Result<(bool)> {
		let req = Request::new(101)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetSetupResutltValue(&self, ) -> Result<(u32)> {
		let req = Request::new(102)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetInternalOffset(&self, ) -> Result<(::nn::TimeSpanType)> {
		let req = Request::new(200)
			.args(())
			;
		let mut res : Response<::nn::TimeSpanType> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SetInternalOffset(&self, unk0: ::nn::TimeSpanType) -> Result<()> {
		let req = Request::new(201)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for ISteadyClock {
	unsafe fn from_kobject(obj: KObject) -> ISteadyClock {
		ISteadyClock(Session::from_kobject(obj))
	}
}
