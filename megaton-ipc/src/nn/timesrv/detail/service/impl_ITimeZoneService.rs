
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct ITimeZoneService(Session);

impl ITimeZoneService {
	pub fn GetDeviceLocationName(&self, ) -> Result<(::nn::time::LocationName)> {
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

	pub fn GetTotalLocationNameCount(&self, ) -> Result<(i32)> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn LoadLocationNameList(&self, unk0: i32, unk2: &mut [::nn::time::LocationName]) -> Result<(i32)> {
		let req = Request::new(3)
			.args(unk0)
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn LoadTimeZoneRule(&self, unk0: ::nn::time::LocationName, unk1: &mut Option<::nn::time::TimeZoneRule>) -> Result<()> {
		let req = Request::new(4)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetTimeZoneRuleVersion(&self, ) -> Result<(::nn::time::TimeZoneRuleVersion)> {
		let req = Request::new(5)
			.args(())
			;
		let mut res : Response<::nn::time::TimeZoneRuleVersion> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn ToCalendarTime(&self, unk0: ::nn::time::PosixTime, unk1: &::nn::time::TimeZoneRule) -> Result<(::nn::time::CalendarTime, ::nn::time::sf::CalendarAdditionalInfo)> {
		let req = Request::new(100)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk2: ::nn::time::CalendarTime,
			unk3: ::nn::time::sf::CalendarAdditionalInfo,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk2.clone(),res.get_raw().unk3.clone()))
	}

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

	pub fn ToPosixTime(&self, unk0: ::nn::time::CalendarTime, unk1: &::nn::time::TimeZoneRule, unk3: &mut [::nn::time::PosixTime]) -> Result<(i32)> {
		let req = Request::new(201)
			.args(unk0)
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn ToPosixTimeWithMyRule(&self, unk0: ::nn::time::CalendarTime, unk2: &mut [::nn::time::PosixTime]) -> Result<(i32)> {
		let req = Request::new(202)
			.args(unk0)
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for ITimeZoneService {
	unsafe fn from_kobject(obj: KObject) -> ITimeZoneService {
		ITimeZoneService(Session::from_kobject(obj))
	}
}
