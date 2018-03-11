
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
	// fn get_client_id(&self, UNKNOWN) -> Result<UNKNOWN>;
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

	// fn get_current_network_profile(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn enumerate_network_interfaces(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn enumerate_network_profiles(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_network_profile(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_network_profile(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn remove_network_profile(&self, unk0: ::nn::util::Uuid) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_scan_data(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_current_ip_address(&self, ) -> Result<::nn::nifm::IpV4Address> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(12)
			.args(())
			;
		let res : Response<::nn::nifm::IpV4Address> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_current_access_point(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn create_temporary_network_profile(&self, UNKNOWN) -> Result<UNKNOWN>;
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

	// fn is_any_internet_request_accepted(&self, UNKNOWN) -> Result<UNKNOWN>;
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

	// fn set_exclusive_client(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_default_ip_setting(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_default_ip_setting(&self, UNKNOWN) -> Result<UNKNOWN>;
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

	// fn get_telemetry_info(&self, UNKNOWN) -> Result<UNKNOWN>;
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
