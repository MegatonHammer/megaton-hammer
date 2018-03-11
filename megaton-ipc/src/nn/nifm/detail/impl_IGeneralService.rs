
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hammer::ipc::IPCBuffer;

#[derive(Debug)]
pub struct IGeneralService(Session);

impl AsRef<Session> for IGeneralService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IGeneralService {
	// fn GetClientId(&self, UNKNOWN) -> Result<UNKNOWN>;
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

	// fn GetCurrentNetworkProfile(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn EnumerateNetworkInterfaces(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn EnumerateNetworkProfiles(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetNetworkProfile(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn SetNetworkProfile(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn RemoveNetworkProfile(&self, unk0: ::nn::util::Uuid) -> Result<()> {
		let req = Request::new(10)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn GetScanData(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetCurrentIpAddress(&self, ) -> Result<::nn::nifm::IpV4Address> {
		let req = Request::new(12)
			.args(())
			;
		let mut res : Response<::nn::nifm::IpV4Address> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn GetCurrentAccessPoint(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn CreateTemporaryNetworkProfile(&self, UNKNOWN) -> Result<UNKNOWN>;
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

	// fn IsAnyInternetRequestAccepted(&self, UNKNOWN) -> Result<UNKNOWN>;
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

	// fn SetExclusiveClient(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetDefaultIpSetting(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn SetDefaultIpSetting(&self, UNKNOWN) -> Result<UNKNOWN>;
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

	pub fn GetTelemetorySystemEventReadableHandle(&self, ) -> Result<KObject> {
		let req = Request::new(31)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn GetTelemetryInfo(&self, UNKNOWN) -> Result<UNKNOWN>;
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
