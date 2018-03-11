
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct ITimeZoneService(Session);

impl AsRef<Session> for ITimeZoneService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ITimeZoneService {
	pub fn get_device_location_name(&self, ) -> Result<::nn::time::LocationName> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let res : Response<::nn::time::LocationName> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_device_location_name(&self, unk0: ::nn::time::LocationName) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_total_location_name_count(&self, ) -> Result<i32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn load_location_name_list(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn load_time_zone_rule(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_time_zone_rule_version(&self, ) -> Result<::nn::time::TimeZoneRuleVersion> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(5)
			.args(())
			;
		let res : Response<::nn::time::TimeZoneRuleVersion> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn to_calendar_time(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn to_calendar_time_with_my_rule(&self, unk0: ::nn::time::PosixTime) -> Result<(::nn::time::CalendarTime, ::nn::time::sf::CalendarAdditionalInfo)> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(101)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: ::nn::time::CalendarTime,
			unk2: ::nn::time::sf::CalendarAdditionalInfo,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().unk2.clone()))
	}

	// fn to_posix_time(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn to_posix_time_with_my_rule(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for ITimeZoneService {
	unsafe fn from_kobject(obj: KObject) -> ITimeZoneService {
		ITimeZoneService(Session::from_kobject(obj))
	}
}
