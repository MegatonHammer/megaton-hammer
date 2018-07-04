
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IGeneralService<T>(T);

impl IGeneralService<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IGeneralService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IGeneralService(domain)),
			Err((sess, err)) => Err((IGeneralService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IGeneralService<Session>> {
		Ok(IGeneralService(self.0.duplicate()?))
	}
}

impl<T> Deref for IGeneralService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IGeneralService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IGeneralService<T> {
	pub fn get_client_id(&self, unk0: &mut ::ipcdefs::nn::nifm::ClientId) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn create_scan_request(&self, ) -> Result<::ipcdefs::nn::nifm::detail::IScanRequest<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn create_request(&self, unk0: i32) -> Result<::ipcdefs::nn::nifm::detail::IRequest<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(4)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_current_network_profile(&self, unk0: &mut ::ipcdefs::nn::nifm::detail::sf::NetworkProfileData) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(5)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn enumerate_network_interfaces(&self, unk0: u32, unk2: &mut [::ipcdefs::nn::nifm::detail::sf::NetworkInterfaceInfo]) -> Result<i32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(6)
			.args(unk0)
			.descriptor(IPCBuffer::from_mut_slice(unk2, 0xa))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn enumerate_network_profiles(&self, unk0: u8, unk2: &mut [::ipcdefs::nn::nifm::detail::sf::NetworkProfileBasicInfo]) -> Result<i32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(7)
			.args(unk0)
			.descriptor(IPCBuffer::from_mut_slice(unk2, 6))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_network_profile(&self, unk0: ::ipcdefs::nn::util::Uuid, unk1: &mut ::ipcdefs::nn::nifm::detail::sf::NetworkProfileData) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(8)
			.args(unk0)
			.descriptor(IPCBuffer::from_mut_ref(unk1, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_network_profile(&self, unk0: &::ipcdefs::nn::nifm::detail::sf::NetworkProfileData) -> Result<::ipcdefs::nn::util::Uuid> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(9)
			.args(())
			.descriptor(IPCBuffer::from_ref(unk0, 0x19))
			;
		let res : Response<::ipcdefs::nn::util::Uuid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn remove_network_profile(&self, unk0: ::ipcdefs::nn::util::Uuid) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(10)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_scan_data(&self, unk1: &mut [::ipcdefs::nn::nifm::detail::sf::AccessPointData]) -> Result<i32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(11)
			.args(())
			.descriptor(IPCBuffer::from_mut_slice(unk1, 6))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_current_ip_address(&self, ) -> Result<::ipcdefs::nn::nifm::IpV4Address> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(12)
			.args(())
			;
		let res : Response<::ipcdefs::nn::nifm::IpV4Address> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_current_access_point(&self, unk0: &mut ::ipcdefs::nn::nifm::detail::sf::AccessPointData) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(13)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn create_temporary_network_profile(&self, unk0: &::ipcdefs::nn::nifm::detail::sf::NetworkProfileData) -> Result<(::ipcdefs::nn::util::Uuid, ::ipcdefs::nn::nifm::detail::INetworkProfile<T>)> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(14)
			.args(())
			.descriptor(IPCBuffer::from_ref(unk0, 0x19))
			;
		let mut res : Response<::ipcdefs::nn::util::Uuid> = self.0.send(req)?;
		Ok((*res.get_raw(),T::from_res(&mut res).into()))
	}

	// fn get_current_ip_config_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn set_wireless_communication_enabled(&self, unk0: bool) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(16)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_wireless_communication_enabled(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(17)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_internet_connection_status(&self, ) -> Result<::ipcdefs::nn::nifm::detail::sf::InternetConnectionStatus> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(18)
			.args(())
			;
		let res : Response<::ipcdefs::nn::nifm::detail::sf::InternetConnectionStatus> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_ethernet_communication_enabled(&self, unk0: bool) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(19)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_ethernet_communication_enabled(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(20)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn is_any_internet_request_accepted(&self, unk0: &::ipcdefs::nn::nifm::ClientId) -> Result<bool> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(21)
			.args(())
			.descriptor(IPCBuffer::from_ref(unk0, 0x19))
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn is_any_foreground_request_accepted(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(22)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn put_to_sleep(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(23)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn wake_up(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(24)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_ssid_list_version(&self, ) -> Result<::ipcdefs::nn::nifm::SsidListVersion> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(25)
			.args(())
			;
		let res : Response<::ipcdefs::nn::nifm::SsidListVersion> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_exclusive_client(&self, unk0: &::ipcdefs::nn::nifm::ClientId) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(26)
			.args(())
			.descriptor(IPCBuffer::from_ref(unk0, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_default_ip_setting(&self, unk0: &mut ::ipcdefs::nn::nifm::IpSettingData) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(27)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_default_ip_setting(&self, unk0: &::ipcdefs::nn::nifm::IpSettingData) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(28)
			.args(())
			.descriptor(IPCBuffer::from_ref(unk0, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_wireless_communication_enabled_for_test(&self, unk0: bool) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(29)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_ethernet_communication_enabled_for_test(&self, unk0: bool) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_telemetory_system_event_readable_handle(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(31)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn get_telemetry_info(&self, unk0: &mut ::ipcdefs::nn::nifm::TelemetryInfo) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(32)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x16))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn confirm_system_availability(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(33)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IGeneralService<T> {
	fn from(obj: T) -> IGeneralService<T> {
		IGeneralService(obj)
	}
}
