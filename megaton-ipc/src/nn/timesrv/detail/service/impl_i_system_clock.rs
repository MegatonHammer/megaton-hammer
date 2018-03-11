
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct ISystemClock(Session);

impl AsRef<Session> for ISystemClock {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ISystemClock {
	pub fn get_current_time(&self, ) -> Result<::nn::time::PosixTime> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let res : Response<::nn::time::PosixTime> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_current_time(&self, unk0: ::nn::time::PosixTime) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_system_clock_context(&self, ) -> Result<::nn::time::SystemClockContext> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let res : Response<::nn::time::SystemClockContext> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_system_clock_context(&self, unk0: ::nn::time::SystemClockContext) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for ISystemClock {
	unsafe fn from_kobject(obj: KObject) -> ISystemClock {
		ISystemClock(Session::from_kobject(obj))
	}
}
