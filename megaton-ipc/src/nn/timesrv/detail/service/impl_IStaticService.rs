
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IStaticService(Session);

impl IStaticService {
	pub fn GetStandardUserSystemClock(&self, ) -> Result<(::nn::timesrv::detail::service::ISystemClock)> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn GetStandardNetworkSystemClock(&self, ) -> Result<(::nn::timesrv::detail::service::ISystemClock)> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn GetStandardSteadyClock(&self, ) -> Result<(::nn::timesrv::detail::service::ISteadyClock)> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn GetTimeZoneService(&self, ) -> Result<(::nn::timesrv::detail::service::ITimeZoneService)> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn GetStandardLocalSystemClock(&self, ) -> Result<(::nn::timesrv::detail::service::ISystemClock)> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn IsStandardUserSystemClockAutomaticCorrectionEnabled(&self, ) -> Result<(bool)> {
		let req = Request::new(100)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SetStandardUserSystemClockAutomaticCorrectionEnabled(&self, unk0: bool) -> Result<()> {
		let req = Request::new(101)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn IsStandardNetworkSystemClockAccuracySufficient(&self, ) -> Result<(bool)> {
		let req = Request::new(200)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IStaticService {
	unsafe fn from_kobject(obj: KObject) -> IStaticService {
		IStaticService(Session::from_kobject(obj))
	}
}
