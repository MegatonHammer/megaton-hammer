
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct ISteadyClock(Session);

impl AsRef<Session> for ISteadyClock {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ISteadyClock {
	pub fn get_current_time_point(&self, ) -> Result<::nn::time::SteadyClockTimePoint> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let res : Response<::nn::time::SteadyClockTimePoint> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_test_offset(&self, ) -> Result<::nn::TimeSpanType> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let res : Response<::nn::TimeSpanType> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_test_offset(&self, unk0: ::nn::TimeSpanType) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_rtc_value(&self, ) -> Result<i64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(100)
			.args(())
			;
		let res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn is_rtc_reset_detected(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(101)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_setup_resutlt_value(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(102)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_internal_offset(&self, ) -> Result<::nn::TimeSpanType> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(200)
			.args(())
			;
		let res : Response<::nn::TimeSpanType> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_internal_offset(&self, unk0: ::nn::TimeSpanType) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(201)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for ISteadyClock {
	unsafe fn from_kobject(obj: KObject) -> ISteadyClock {
		ISteadyClock(Session::from_kobject(obj))
	}
}
