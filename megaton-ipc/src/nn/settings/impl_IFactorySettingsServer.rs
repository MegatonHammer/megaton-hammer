
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IFactorySettingsServer(Session);

impl IFactorySettingsServer {
	pub fn get_service() -> Result<IFactorySettingsServer> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"set:cal\0").map(|s| unsafe { IFactorySettingsServer::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl IFactorySettingsServer {
	pub fn GetBluetoothBdAddress(&self, ) -> Result<::nn::settings::factory::BdAddress> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<::nn::settings::factory::BdAddress> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetConfigurationId1(&self, ) -> Result<::nn::settings::factory::ConfigurationId1> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<::nn::settings::factory::ConfigurationId1> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetAccelerometerOffset(&self, ) -> Result<::nn::settings::factory::AccelerometerOffset> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<::nn::settings::factory::AccelerometerOffset> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetAccelerometerScale(&self, ) -> Result<::nn::settings::factory::AccelerometerScale> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<::nn::settings::factory::AccelerometerScale> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetGyroscopeOffset(&self, ) -> Result<::nn::settings::factory::GyroscopeOffset> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<::nn::settings::factory::GyroscopeOffset> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetGyroscopeScale(&self, ) -> Result<::nn::settings::factory::GyroscopeScale> {
		let req = Request::new(5)
			.args(())
			;
		let mut res : Response<::nn::settings::factory::GyroscopeScale> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetWirelessLanMacAddress(&self, ) -> Result<::nn::settings::factory::MacAddress> {
		let req = Request::new(6)
			.args(())
			;
		let mut res : Response<::nn::settings::factory::MacAddress> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetWirelessLanCountryCodeCount(&self, ) -> Result<i32> {
		let req = Request::new(7)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetWirelessLanCountryCodes(&self, unk1: &mut [::nn::settings::factory::CountryCode]) -> Result<i32> {
		let req = Request::new(8)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetSerialNumber(&self, ) -> Result<::nn::settings::factory::SerialNumber> {
		let req = Request::new(9)
			.args(())
			;
		let mut res : Response<::nn::settings::factory::SerialNumber> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SetInitialSystemAppletProgramId(&self, unk0: ::nn::ncm::ProgramId) -> Result<()> {
		let req = Request::new(10)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetOverlayDispProgramId(&self, unk0: ::nn::ncm::ProgramId) -> Result<()> {
		let req = Request::new(11)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetBatteryLot(&self, ) -> Result<::nn::settings::factory::BatteryLot> {
		let req = Request::new(12)
			.args(())
			;
		let mut res : Response<::nn::settings::factory::BatteryLot> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetEciDeviceCertificate(&self, unk0: &mut Option<::nn::settings::factory::EccB233DeviceCertificate>) -> Result<()> {
		let req = Request::new(14)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetEticketDeviceCertificate(&self, unk0: &mut Option<::nn::settings::factory::Rsa2048DeviceCertificate>) -> Result<()> {
		let req = Request::new(15)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetSslKey(&self, unk0: &mut Option<::nn::settings::factory::SslKey>) -> Result<()> {
		let req = Request::new(16)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetSslCertificate(&self, unk0: &mut Option<::nn::settings::factory::SslCertificate>) -> Result<()> {
		let req = Request::new(17)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetGameCardKey(&self, unk0: &mut Option<::nn::settings::factory::GameCardKey>) -> Result<()> {
		let req = Request::new(18)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetGameCardCertificate(&self, unk0: &mut Option<::nn::settings::factory::GameCardCertificate>) -> Result<()> {
		let req = Request::new(19)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetEciDeviceKey(&self, ) -> Result<::nn::settings::factory::EccB233DeviceKey> {
		let req = Request::new(20)
			.args(())
			;
		let mut res : Response<::nn::settings::factory::EccB233DeviceKey> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetEticketDeviceKey(&self, unk0: &mut Option<::nn::settings::factory::Rsa2048DeviceKey>) -> Result<()> {
		let req = Request::new(21)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetSpeakerParameter(&self, ) -> Result<::nn::settings::factory::SpeakerParameter> {
		let req = Request::new(22)
			.args(())
			;
		let mut res : Response<::nn::settings::factory::SpeakerParameter> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn GetLcdVendorId(&self, ) -> Result<()> {
		let req = Request::new(23)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IFactorySettingsServer {
	unsafe fn from_kobject(obj: KObject) -> IFactorySettingsServer {
		IFactorySettingsServer(Session::from_kobject(obj))
	}
}
