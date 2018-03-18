
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IFactorySettingsServer(Session);

impl IFactorySettingsServer {
	pub fn new() -> Result<Arc<IFactorySettingsServer>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IFactorySettingsServer>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"set:cal\0") {
			let ret = Arc::new(IFactorySettingsServer(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"set:cal\0").map(|s| Arc::new(unsafe { IFactorySettingsServer::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IFactorySettingsServer {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IFactorySettingsServer {
	pub fn get_bluetooth_bd_address(&self, ) -> Result<::nn::settings::factory::BdAddress> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let res : Response<::nn::settings::factory::BdAddress> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_configuration_id1(&self, ) -> Result<::nn::settings::factory::ConfigurationId1> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let res : Response<::nn::settings::factory::ConfigurationId1> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_accelerometer_offset(&self, ) -> Result<::nn::settings::factory::AccelerometerOffset> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let res : Response<::nn::settings::factory::AccelerometerOffset> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_accelerometer_scale(&self, ) -> Result<::nn::settings::factory::AccelerometerScale> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			;
		let res : Response<::nn::settings::factory::AccelerometerScale> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_gyroscope_offset(&self, ) -> Result<::nn::settings::factory::GyroscopeOffset> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(())
			;
		let res : Response<::nn::settings::factory::GyroscopeOffset> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_gyroscope_scale(&self, ) -> Result<::nn::settings::factory::GyroscopeScale> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(5)
			.args(())
			;
		let res : Response<::nn::settings::factory::GyroscopeScale> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_wireless_lan_mac_address(&self, ) -> Result<::nn::settings::factory::MacAddress> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(6)
			.args(())
			;
		let res : Response<::nn::settings::factory::MacAddress> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_wireless_lan_country_code_count(&self, ) -> Result<i32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(7)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_wireless_lan_country_codes(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_serial_number(&self, ) -> Result<::nn::settings::factory::SerialNumber> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(9)
			.args(())
			;
		let res : Response<::nn::settings::factory::SerialNumber> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_initial_system_applet_program_id(&self, unk0: ::nn::ncm::ProgramId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_overlay_disp_program_id(&self, unk0: ::nn::ncm::ProgramId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(11)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_battery_lot(&self, ) -> Result<::nn::settings::factory::BatteryLot> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(12)
			.args(())
			;
		let res : Response<::nn::settings::factory::BatteryLot> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_eci_device_certificate(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_eticket_device_certificate(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_ssl_key(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_ssl_certificate(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_game_card_key(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_game_card_certificate(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_eci_device_key(&self, ) -> Result<::nn::settings::factory::EccB233DeviceKey> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(20)
			.args(())
			;
		let res : Response<::nn::settings::factory::EccB233DeviceKey> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_eticket_device_key(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_speaker_parameter(&self, ) -> Result<::nn::settings::factory::SpeakerParameter> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(22)
			.args(())
			;
		let res : Response<::nn::settings::factory::SpeakerParameter> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn get_lcd_vendor_id(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(23)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IFactorySettingsServer {
	unsafe fn from_kobject(obj: KObject) -> IFactorySettingsServer {
		IFactorySettingsServer(Session::from_kobject(obj))
	}
}
