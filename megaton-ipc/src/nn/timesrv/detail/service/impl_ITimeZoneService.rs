
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct ITimeZoneService(Session);

impl AsRef<Session> for ITimeZoneService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ITimeZoneService {
	pub fn GetDeviceLocationName(&self, ) -> Result<::nn::time::LocationName> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<::nn::time::LocationName> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SetDeviceLocationName(&self, unk0: ::nn::time::LocationName) -> Result<()> {
		let req = Request::new(1)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetTotalLocationNameCount(&self, ) -> Result<i32> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn LoadLocationNameList(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn LoadTimeZoneRule(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetTimeZoneRuleVersion(&self, ) -> Result<::nn::time::TimeZoneRuleVersion> {
		let req = Request::new(5)
			.args(())
			;
		let mut res : Response<::nn::time::TimeZoneRuleVersion> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn ToCalendarTime(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn ToCalendarTimeWithMyRule(&self, unk0: ::nn::time::PosixTime) -> Result<(::nn::time::CalendarTime, ::nn::time::sf::CalendarAdditionalInfo)> {
		let req = Request::new(101)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: ::nn::time::CalendarTime,
			unk2: ::nn::time::sf::CalendarAdditionalInfo,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().unk2.clone()))
	}

	// fn ToPosixTime(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn ToPosixTimeWithMyRule(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for ITimeZoneService {
	unsafe fn from_kobject(obj: KObject) -> ITimeZoneService {
		ITimeZoneService(Session::from_kobject(obj))
	}
}
