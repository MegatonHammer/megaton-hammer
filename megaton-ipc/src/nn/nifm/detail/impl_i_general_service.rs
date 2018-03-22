
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IGeneralService(Session);

impl AsRef<Session> for IGeneralService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IGeneralService {
	pub fn get_client_id(&self, unk0: &mut ::nn::nifm::ClientId) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn create_scan_request(&self, ) -> Result<::nn::nifm::detail::IScanRequest> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn create_request(&self, unk0: i32) -> Result<::nn::nifm::detail::IRequest> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn get_current_network_profile(&self, unk0: &mut ::nn::nifm::detail::sf::NetworkProfileData) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(5)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn enumerate_network_interfaces(&self, unk0: u32, unk2: &mut [::nn::nifm::detail::sf::NetworkInterfaceInfo]) -> Result<i32> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(6)
			.args(unk0)
			.descriptor(IPCBuffer::from_mut_slice(unk2, 0xa))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn enumerate_network_profiles(&self, unk0: u8, unk2: &mut [::nn::nifm::detail::sf::NetworkProfileBasicInfo]) -> Result<i32> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(7)
			.args(unk0)
			.descriptor(IPCBuffer::from_mut_slice(unk2, 6))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_network_profile(&self, unk0: ::nn::util::Uuid, unk1: &mut ::nn::nifm::detail::sf::NetworkProfileData) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(8)
			.args(unk0)
			.descriptor(IPCBuffer::from_mut_ref(unk1, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_network_profile(&self, unk0: &::nn::nifm::detail::sf::NetworkProfileData) -> Result<::nn::util::Uuid> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(9)
			.args(())
			.descriptor(IPCBuffer::from_ref(unk0, 0x19))
			;
		let res : Response<::nn::util::Uuid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn remove_network_profile(&self, unk0: ::nn::util::Uuid) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_scan_data(&self, unk1: &mut [::nn::nifm::detail::sf::AccessPointData]) -> Result<i32> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(11)
			.args(())
			.descriptor(IPCBuffer::from_mut_slice(unk1, 6))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_current_ip_address(&self, ) -> Result<::nn::nifm::IpV4Address> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(12)
			.args(())
			;
		let res : Response<::nn::nifm::IpV4Address> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_current_access_point(&self, unk0: &mut ::nn::nifm::detail::sf::AccessPointData) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(13)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn create_temporary_network_profile(&self, unk0: &::nn::nifm::detail::sf::NetworkProfileData) -> Result<(::nn::util::Uuid, ::nn::nifm::detail::INetworkProfile)> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(14)
			.args(())
			.descriptor(IPCBuffer::from_ref(unk0, 0x19))
			;
		let mut res : Response<::nn::util::Uuid> = self.0.send(req)?;
		Ok((*res.get_raw(),unsafe { FromKObject::from_kobject(res.pop_handle()) }))
	}

	// fn get_current_ip_config_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn set_wireless_communication_enabled(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(16)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_wireless_communication_enabled(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(17)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_internet_connection_status(&self, ) -> Result<::nn::nifm::detail::sf::InternetConnectionStatus> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(18)
			.args(())
			;
		let res : Response<::nn::nifm::detail::sf::InternetConnectionStatus> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_ethernet_communication_enabled(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(19)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_ethernet_communication_enabled(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(20)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn is_any_internet_request_accepted(&self, unk0: &::nn::nifm::ClientId) -> Result<bool> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(21)
			.args(())
			.descriptor(IPCBuffer::from_ref(unk0, 0x19))
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn is_any_foreground_request_accepted(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(22)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn put_to_sleep(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(23)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn wake_up(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(24)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_ssid_list_version(&self, ) -> Result<::nn::nifm::SsidListVersion> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(25)
			.args(())
			;
		let res : Response<::nn::nifm::SsidListVersion> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_exclusive_client(&self, unk0: &::nn::nifm::ClientId) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(26)
			.args(())
			.descriptor(IPCBuffer::from_ref(unk0, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_default_ip_setting(&self, unk0: &mut ::nn::nifm::IpSettingData) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(27)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_default_ip_setting(&self, unk0: &::nn::nifm::IpSettingData) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(28)
			.args(())
			.descriptor(IPCBuffer::from_ref(unk0, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_wireless_communication_enabled_for_test(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(29)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_ethernet_communication_enabled_for_test(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(30)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_telemetory_system_event_readable_handle(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(31)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn get_telemetry_info(&self, unk0: &mut ::nn::nifm::TelemetryInfo) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(32)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x16))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn confirm_system_availability(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(33)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IGeneralService {
	unsafe fn from_kobject(obj: KObject) -> IGeneralService {
		IGeneralService(Session::from_kobject(obj))
	}
}
