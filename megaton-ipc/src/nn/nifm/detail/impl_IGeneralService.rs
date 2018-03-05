
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IGeneralService(Session);

impl IGeneralService {
	pub fn GetClientId(&self, unk0: &mut Option<::nn::nifm::ClientId>) -> Result<()> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn CreateScanRequest(&self, ) -> Result<::nn::nifm::detail::IScanRequest> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}
	pub fn CreateRequest(&self, unk0: i32) -> Result<::nn::nifm::detail::IRequest> {
		let req = Request::new(4)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}
	pub fn GetCurrentNetworkProfile(&self, unk0: &mut Option<::nn::nifm::detail::sf::NetworkProfileData>) -> Result<()> {
		let req = Request::new(5)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn EnumerateNetworkInterfaces(&self, unk0: u32, unk2: &mut [::nn::nifm::detail::sf::NetworkInterfaceInfo]) -> Result<i32> {
		let req = Request::new(6)
			.args(unk0)
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn EnumerateNetworkProfiles(&self, unk0: u8, unk2: &mut [::nn::nifm::detail::sf::NetworkProfileBasicInfo]) -> Result<i32> {
		let req = Request::new(7)
			.args(unk0)
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetNetworkProfile(&self, unk0: ::nn::util::Uuid, unk1: &mut Option<::nn::nifm::detail::sf::NetworkProfileData>) -> Result<()> {
		let req = Request::new(8)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn SetNetworkProfile(&self, unk0: &::nn::nifm::detail::sf::NetworkProfileData) -> Result<::nn::util::Uuid> {
		let req = Request::new(9)
			.args(())
			;
		let mut res : Response<::nn::util::Uuid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn RemoveNetworkProfile(&self, unk0: ::nn::util::Uuid) -> Result<()> {
		let req = Request::new(10)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetScanData(&self, unk1: &mut [::nn::nifm::detail::sf::AccessPointData]) -> Result<i32> {
		let req = Request::new(11)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetCurrentIpAddress(&self, ) -> Result<::nn::nifm::IpV4Address> {
		let req = Request::new(12)
			.args(())
			;
		let mut res : Response<::nn::nifm::IpV4Address> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetCurrentAccessPoint(&self, unk0: &mut Option<::nn::nifm::detail::sf::AccessPointData>) -> Result<()> {
		let req = Request::new(13)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn CreateTemporaryNetworkProfile(&self, unk0: &::nn::nifm::detail::sf::NetworkProfileData) -> Result<(::nn::util::Uuid, ::nn::nifm::detail::INetworkProfile)> {
		let req = Request::new(14)
			.args(())
			;
		let mut res : Response<::nn::util::Uuid> = self.0.send(req)?;
		Ok((*res.get_raw(),unsafe { FromKObject::from_kobject(res.pop_handle()) }))
	}
	// fn GetCurrentIpConfigInfo(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn SetWirelessCommunicationEnabled(&self, unk0: bool) -> Result<()> {
		let req = Request::new(16)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn IsWirelessCommunicationEnabled(&self, ) -> Result<bool> {
		let req = Request::new(17)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetInternetConnectionStatus(&self, ) -> Result<::nn::nifm::detail::sf::InternetConnectionStatus> {
		let req = Request::new(18)
			.args(())
			;
		let mut res : Response<::nn::nifm::detail::sf::InternetConnectionStatus> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetEthernetCommunicationEnabled(&self, unk0: bool) -> Result<()> {
		let req = Request::new(19)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn IsEthernetCommunicationEnabled(&self, ) -> Result<bool> {
		let req = Request::new(20)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn IsAnyInternetRequestAccepted(&self, unk0: &::nn::nifm::ClientId) -> Result<bool> {
		let req = Request::new(21)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn IsAnyForegroundRequestAccepted(&self, ) -> Result<bool> {
		let req = Request::new(22)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn PutToSleep(&self, ) -> Result<()> {
		let req = Request::new(23)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn WakeUp(&self, ) -> Result<()> {
		let req = Request::new(24)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetSsidListVersion(&self, ) -> Result<::nn::nifm::SsidListVersion> {
		let req = Request::new(25)
			.args(())
			;
		let mut res : Response<::nn::nifm::SsidListVersion> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetExclusiveClient(&self, unk0: &::nn::nifm::ClientId) -> Result<()> {
		let req = Request::new(26)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetDefaultIpSetting(&self, unk0: &mut Option<::nn::nifm::IpSettingData>) -> Result<()> {
		let req = Request::new(27)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn SetDefaultIpSetting(&self, unk0: &::nn::nifm::IpSettingData) -> Result<()> {
		let req = Request::new(28)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn SetWirelessCommunicationEnabledForTest(&self, unk0: bool) -> Result<()> {
		let req = Request::new(29)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn SetEthernetCommunicationEnabledForTest(&self, unk0: bool) -> Result<()> {
		let req = Request::new(30)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn GetTelemetorySystemEventReadableHandle(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetTelemetryInfo(&self, unk0: &mut Option<::nn::nifm::TelemetryInfo>) -> Result<()> {
		let req = Request::new(32)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn ConfirmSystemAvailability(&self, ) -> Result<()> {
		let req = Request::new(33)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
}

impl FromKObject for IGeneralService {
	unsafe fn from_kobject(obj: KObject) -> IGeneralService {
		IGeneralService(Session::from_kobject(obj))
	}
}
