
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IFactorySettingsServer<T>(T);

impl IFactorySettingsServer<Session> {
	pub fn raw_new() -> Result<IFactorySettingsServer<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"set:cal\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IFactorySettingsServer<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IFactorySettingsServer<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"set:cal\0") {
			let ret = Arc::new(IFactorySettingsServer(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IFactorySettingsServer<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IFactorySettingsServer(domain)),
			Err((sess, err)) => Err((IFactorySettingsServer(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IFactorySettingsServer<Session>> {
		Ok(IFactorySettingsServer(self.0.duplicate()?))
	}
}

impl<T> Deref for IFactorySettingsServer<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IFactorySettingsServer<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IFactorySettingsServer<T> {
	pub fn get_bluetooth_bd_address(&self, ) -> Result<::ipcdefs::nn::settings::factory::BdAddress> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let res : Response<::ipcdefs::nn::settings::factory::BdAddress> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_configuration_id1(&self, ) -> Result<::ipcdefs::nn::settings::factory::ConfigurationId1> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let res : Response<::ipcdefs::nn::settings::factory::ConfigurationId1> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_accelerometer_offset(&self, ) -> Result<::ipcdefs::nn::settings::factory::AccelerometerOffset> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let res : Response<::ipcdefs::nn::settings::factory::AccelerometerOffset> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_accelerometer_scale(&self, ) -> Result<::ipcdefs::nn::settings::factory::AccelerometerScale> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(3)
			.args(())
			;
		let res : Response<::ipcdefs::nn::settings::factory::AccelerometerScale> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_gyroscope_offset(&self, ) -> Result<::ipcdefs::nn::settings::factory::GyroscopeOffset> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(4)
			.args(())
			;
		let res : Response<::ipcdefs::nn::settings::factory::GyroscopeOffset> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_gyroscope_scale(&self, ) -> Result<::ipcdefs::nn::settings::factory::GyroscopeScale> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(5)
			.args(())
			;
		let res : Response<::ipcdefs::nn::settings::factory::GyroscopeScale> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_wireless_lan_mac_address(&self, ) -> Result<::ipcdefs::nn::settings::factory::MacAddress> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(6)
			.args(())
			;
		let res : Response<::ipcdefs::nn::settings::factory::MacAddress> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_wireless_lan_country_code_count(&self, ) -> Result<i32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(7)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_wireless_lan_country_codes(&self, unk1: &mut [::ipcdefs::nn::settings::factory::CountryCode]) -> Result<i32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(8)
			.args(())
			.descriptor(IPCBuffer::from_mut_slice(unk1, 0xa))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_serial_number(&self, ) -> Result<::ipcdefs::nn::settings::factory::SerialNumber> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(9)
			.args(())
			;
		let res : Response<::ipcdefs::nn::settings::factory::SerialNumber> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_initial_system_applet_program_id(&self, unk0: ::ipcdefs::nn::ncm::ProgramId) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(10)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_overlay_disp_program_id(&self, unk0: ::ipcdefs::nn::ncm::ProgramId) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(11)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_battery_lot(&self, ) -> Result<::ipcdefs::nn::settings::factory::BatteryLot> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(12)
			.args(())
			;
		let res : Response<::ipcdefs::nn::settings::factory::BatteryLot> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_eci_device_certificate(&self, unk0: &mut ::ipcdefs::nn::settings::factory::EccB233DeviceCertificate) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(14)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x16))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_eticket_device_certificate(&self, unk0: &mut ::ipcdefs::nn::settings::factory::Rsa2048DeviceCertificate) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(15)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x16))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_ssl_key(&self, unk0: &mut ::ipcdefs::nn::settings::factory::SslKey) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(16)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x16))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_ssl_certificate(&self, unk0: &mut ::ipcdefs::nn::settings::factory::SslCertificate) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(17)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x16))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_game_card_key(&self, unk0: &mut ::ipcdefs::nn::settings::factory::GameCardKey) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(18)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x16))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_game_card_certificate(&self, unk0: &mut ::ipcdefs::nn::settings::factory::GameCardCertificate) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(19)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x16))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_eci_device_key(&self, ) -> Result<::ipcdefs::nn::settings::factory::EccB233DeviceKey> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(20)
			.args(())
			;
		let res : Response<::ipcdefs::nn::settings::factory::EccB233DeviceKey> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_eticket_device_key(&self, unk0: &mut ::ipcdefs::nn::settings::factory::Rsa2048DeviceKey) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(21)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x16))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_speaker_parameter(&self, ) -> Result<::ipcdefs::nn::settings::factory::SpeakerParameter> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(22)
			.args(())
			;
		let res : Response<::ipcdefs::nn::settings::factory::SpeakerParameter> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn get_lcd_vendor_id(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(23)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn get_unknown_key1(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(24)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn get_unknown_key0(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(25)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn get_amiibo_key(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(26)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn get_amiibo_ecqv_certificate(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(27)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn get_amiibo_ecdsa_certificate(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(28)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn get_amiibo_ecqvbls_key(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(29)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn get_amiibo_ecqvbls_certificate(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn get_amiibo_ecqvbls_root_certificate(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(31)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn get_unknown_id(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(32)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IFactorySettingsServer<T> {
	fn from(obj: T) -> IFactorySettingsServer<T> {
		IFactorySettingsServer(obj)
	}
}
