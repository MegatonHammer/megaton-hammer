
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct ITimeZoneService<T>(T);

impl ITimeZoneService<Session> {
	pub fn to_domain(self) -> ::core::result::Result<ITimeZoneService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ITimeZoneService(domain)),
			Err((sess, err)) => Err((ITimeZoneService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ITimeZoneService<Session>> {
		Ok(ITimeZoneService(self.0.duplicate()?))
	}
}

impl<T> Deref for ITimeZoneService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ITimeZoneService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ITimeZoneService<T> {
	pub fn get_device_location_name(&self, ) -> Result<::nn::time::LocationName> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let res : Response<::nn::time::LocationName> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_device_location_name(&self, unk0: ::nn::time::LocationName) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_total_location_name_count(&self, ) -> Result<i32> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn load_location_name_list(&self, unk0: i32, unk2: &mut [::nn::time::LocationName]) -> Result<i32> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(3)
			.args(unk0)
			.descriptor(IPCBuffer::from_mut_slice(unk2, 6))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn load_time_zone_rule(&self, unk0: ::nn::time::LocationName, unk1: &mut ::nn::time::TimeZoneRule) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(4)
			.args(unk0)
			.descriptor(IPCBuffer::from_mut_ref(unk1, 0x16))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_time_zone_rule_version(&self, ) -> Result<::nn::time::TimeZoneRuleVersion> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(5)
			.args(())
			;
		let res : Response<::nn::time::TimeZoneRuleVersion> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn to_calendar_time(&self, unk0: ::nn::time::PosixTime, unk1: &::nn::time::TimeZoneRule) -> Result<(::nn::time::CalendarTime, ::nn::time::sf::CalendarAdditionalInfo)> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(100)
			.args(unk0)
			.descriptor(IPCBuffer::from_ref(unk1, 0x15))
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk2: ::nn::time::CalendarTime,
			unk3: ::nn::time::sf::CalendarAdditionalInfo,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk2.clone(),res.get_raw().unk3.clone()))
	}

	pub fn to_calendar_time_with_my_rule(&self, unk0: ::nn::time::PosixTime) -> Result<(::nn::time::CalendarTime, ::nn::time::sf::CalendarAdditionalInfo)> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(101)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: ::nn::time::CalendarTime,
			unk2: ::nn::time::sf::CalendarAdditionalInfo,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().unk2.clone()))
	}

	pub fn to_posix_time(&self, unk0: ::nn::time::CalendarTime, unk1: &::nn::time::TimeZoneRule, unk3: &mut [::nn::time::PosixTime]) -> Result<i32> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 2], [_; 0], [_; 0]> = Request::new(201)
			.args(unk0)
			.descriptor(IPCBuffer::from_ref(unk1, 0x15))
			.descriptor(IPCBuffer::from_mut_slice(unk3, 0xa))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn to_posix_time_with_my_rule(&self, unk0: ::nn::time::CalendarTime, unk2: &mut [::nn::time::PosixTime]) -> Result<i32> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(202)
			.args(unk0)
			.descriptor(IPCBuffer::from_mut_slice(unk2, 0xa))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl<T: Object> From<T> for ITimeZoneService<T> {
	fn from(obj: T) -> ITimeZoneService<T> {
		ITimeZoneService(obj)
	}
}
