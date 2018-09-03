
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::sync::Arc;

#[derive(Debug)]
pub struct ISystemSettingsServer<T>(T);

impl ISystemSettingsServer<Session> {
	pub fn raw_new() -> Result<ISystemSettingsServer<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"set:sys\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<ISystemSettingsServer<Session>>> {
		use alloc::sync::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<ISystemSettingsServer<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"set:sys\0") {
			let ret = Arc::new(ISystemSettingsServer(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<ISystemSettingsServer<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ISystemSettingsServer(domain)),
			Err((sess, err)) => Err((ISystemSettingsServer(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ISystemSettingsServer<Session>> {
		Ok(ISystemSettingsServer(self.0.duplicate()?))
	}
}

impl<T> Deref for ISystemSettingsServer<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ISystemSettingsServer<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ISystemSettingsServer<T> {
	pub fn set_language_code(&self, unk0: ::ipcdefs::nn::settings::LanguageCode) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_network_settings(&self, unk0: &[::ipcdefs::nn::settings::system::NetworkSettings]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			.descriptor(IPCBuffer::from_slice(unk0, 5))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_network_settings(&self, unk1: &mut [::ipcdefs::nn::settings::system::NetworkSettings]) -> Result<i32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			.descriptor(IPCBuffer::from_mut_slice(unk1, 6))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_firmware_version(&self, unk0: &mut ::ipcdefs::nn::settings::system::FirmwareVersion) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(3)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn get_firmware_version2(&self, unk0: &mut ::ipcdefs::nn::settings::system::FirmwareVersion) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(4)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_firmware_version_digest(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_lock_screen_flag(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(7)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_lock_screen_flag(&self, unk0: bool) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(8)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_backlight_settings(&self, ) -> Result<::ipcdefs::nn::settings::system::BacklightSettings> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(9)
			.args(())
			;
		let res : Response<::ipcdefs::nn::settings::system::BacklightSettings> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_backlight_settings(&self, unk0: ::ipcdefs::nn::settings::system::BacklightSettings) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(10)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_bluetooth_devices_settings(&self, unk0: &[::ipcdefs::nn::settings::system::BluetoothDevicesSettings]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(11)
			.args(())
			.descriptor(IPCBuffer::from_slice(unk0, 5))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_bluetooth_devices_settings(&self, unk1: &mut [::ipcdefs::nn::settings::system::BluetoothDevicesSettings]) -> Result<i32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(12)
			.args(())
			.descriptor(IPCBuffer::from_mut_slice(unk1, 6))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_external_steady_clock_source_id(&self, ) -> Result<::ipcdefs::nn::util::Uuid> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(13)
			.args(())
			;
		let res : Response<::ipcdefs::nn::util::Uuid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_external_steady_clock_source_id(&self, unk0: ::ipcdefs::nn::util::Uuid) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(14)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_user_system_clock_context(&self, ) -> Result<::ipcdefs::nn::time::SystemClockContext> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(15)
			.args(())
			;
		let res : Response<::ipcdefs::nn::time::SystemClockContext> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_user_system_clock_context(&self, unk0: ::ipcdefs::nn::time::SystemClockContext) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(16)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_account_settings(&self, ) -> Result<::ipcdefs::nn::settings::system::AccountSettings> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(17)
			.args(())
			;
		let res : Response<::ipcdefs::nn::settings::system::AccountSettings> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_account_settings(&self, unk0: ::ipcdefs::nn::settings::system::AccountSettings) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(18)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_audio_volume(&self, unk0: i32) -> Result<::ipcdefs::nn::settings::system::AudioVolume> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(19)
			.args(unk0)
			;
		let res : Response<::ipcdefs::nn::settings::system::AudioVolume> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_audio_volume(&self, unk0: ::ipcdefs::nn::settings::system::AudioVolume, unk1: i32) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::settings::system::AudioVolume,
			unk1: i32,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(20)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_eula_versions(&self, unk1: &mut [::ipcdefs::nn::settings::system::EulaVersion]) -> Result<i32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(21)
			.args(())
			.descriptor(IPCBuffer::from_mut_slice(unk1, 6))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_eula_versions(&self, unk0: &[::ipcdefs::nn::settings::system::EulaVersion]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(22)
			.args(())
			.descriptor(IPCBuffer::from_slice(unk0, 5))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_color_set_id(&self, ) -> Result<i32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(23)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_color_set_id(&self, unk0: i32) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(24)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_console_information_upload_flag(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(25)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_console_information_upload_flag(&self, unk0: bool) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(26)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_automatic_application_download_flag(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(27)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_automatic_application_download_flag(&self, unk0: bool) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(28)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_notification_settings(&self, ) -> Result<::ipcdefs::nn::settings::system::NotificationSettings> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(29)
			.args(())
			;
		let res : Response<::ipcdefs::nn::settings::system::NotificationSettings> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_notification_settings(&self, unk0: ::ipcdefs::nn::settings::system::NotificationSettings) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_account_notification_settings(&self, unk1: &mut [::ipcdefs::nn::settings::system::AccountNotificationSettings]) -> Result<i32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(31)
			.args(())
			.descriptor(IPCBuffer::from_mut_slice(unk1, 6))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_account_notification_settings(&self, unk0: &[::ipcdefs::nn::settings::system::AccountNotificationSettings]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(32)
			.args(())
			.descriptor(IPCBuffer::from_slice(unk0, 5))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_vibration_master_volume(&self, ) -> Result<f32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(35)
			.args(())
			;
		let res : Response<f32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_vibration_master_volume(&self, unk0: f32) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(36)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_settings_item_value_size(&self, unk0: &::ipcdefs::nn::settings::SettingsName, unk1: &::ipcdefs::nn::settings::SettingsItemKey) -> Result<u64> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 2], [_; 0], [_; 0]> = Request::new(37)
			.args(())
			.descriptor(IPCBuffer::from_ref(unk0, 0x19))
			.descriptor(IPCBuffer::from_ref(unk1, 0x19))
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_settings_item_value(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_tv_settings(&self, ) -> Result<::ipcdefs::nn::settings::system::TvSettings> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(39)
			.args(())
			;
		let res : Response<::ipcdefs::nn::settings::system::TvSettings> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_tv_settings(&self, unk0: ::ipcdefs::nn::settings::system::TvSettings) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(40)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_edid(&self, unk0: &mut ::ipcdefs::nn::settings::system::Edid) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(41)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_edid(&self, unk0: &::ipcdefs::nn::settings::system::Edid) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(42)
			.args(())
			.descriptor(IPCBuffer::from_ref(unk0, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_audio_output_mode(&self, unk0: i32) -> Result<i32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(43)
			.args(unk0)
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_audio_output_mode(&self, unk0: i32, unk1: i32) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: i32,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(44)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_force_mute_on_headphone_removed(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(45)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_force_mute_on_headphone_removed(&self, unk0: bool) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(46)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_quest_flag(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(47)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_quest_flag(&self, unk0: bool) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(48)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_data_deletion_settings(&self, ) -> Result<::ipcdefs::nn::settings::system::DataDeletionSettings> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(49)
			.args(())
			;
		let res : Response<::ipcdefs::nn::settings::system::DataDeletionSettings> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_data_deletion_settings(&self, unk0: ::ipcdefs::nn::settings::system::DataDeletionSettings) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(50)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_initial_system_applet_program_id(&self, ) -> Result<::ipcdefs::nn::ncm::ProgramId> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(51)
			.args(())
			;
		let res : Response<::ipcdefs::nn::ncm::ProgramId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_overlay_disp_program_id(&self, ) -> Result<::ipcdefs::nn::ncm::ProgramId> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(52)
			.args(())
			;
		let res : Response<::ipcdefs::nn::ncm::ProgramId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_device_time_zone_location_name(&self, ) -> Result<::ipcdefs::nn::time::LocationName> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(53)
			.args(())
			;
		let res : Response<::ipcdefs::nn::time::LocationName> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_device_time_zone_location_name(&self, unk0: ::ipcdefs::nn::time::LocationName) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(54)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_wireless_certification_file_size(&self, ) -> Result<u64> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(55)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_wireless_certification_file(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn set_region_code(&self, unk0: i32) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(57)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_network_system_clock_context(&self, ) -> Result<::ipcdefs::nn::time::SystemClockContext> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(58)
			.args(())
			;
		let res : Response<::ipcdefs::nn::time::SystemClockContext> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_network_system_clock_context(&self, unk0: ::ipcdefs::nn::time::SystemClockContext) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(59)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_user_system_clock_automatic_correction_enabled(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(60)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_user_system_clock_automatic_correction_enabled(&self, unk0: bool) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(61)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_debug_mode_flag(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(62)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_primary_album_storage(&self, ) -> Result<i32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(63)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_primary_album_storage(&self, unk0: i32) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(64)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_usb30_enable_flag(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(65)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_usb30_enable_flag(&self, unk0: bool) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(66)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_battery_lot(&self, ) -> Result<::ipcdefs::nn::settings::system::BatteryLot> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(67)
			.args(())
			;
		let res : Response<::ipcdefs::nn::settings::system::BatteryLot> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_serial_number(&self, ) -> Result<::ipcdefs::nn::settings::system::SerialNumber> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(68)
			.args(())
			;
		let res : Response<::ipcdefs::nn::settings::system::SerialNumber> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_nfc_enable_flag(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(69)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_nfc_enable_flag(&self, unk0: bool) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(70)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_sleep_settings(&self, ) -> Result<::ipcdefs::nn::settings::system::SleepSettings> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(71)
			.args(())
			;
		let res : Response<::ipcdefs::nn::settings::system::SleepSettings> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_sleep_settings(&self, unk0: ::ipcdefs::nn::settings::system::SleepSettings) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(72)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_wireless_lan_enable_flag(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(73)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_wireless_lan_enable_flag(&self, unk0: bool) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(74)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_initial_launch_settings(&self, ) -> Result<::ipcdefs::nn::settings::system::InitialLaunchSettings> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(75)
			.args(())
			;
		let res : Response<::ipcdefs::nn::settings::system::InitialLaunchSettings> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_initial_launch_settings(&self, unk0: ::ipcdefs::nn::settings::system::InitialLaunchSettings) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(76)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_device_nick_name(&self, unk0: &mut ::ipcdefs::nn::settings::system::DeviceNickName) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(77)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x16))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_device_nick_name(&self, unk0: &::ipcdefs::nn::settings::system::DeviceNickName) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(78)
			.args(())
			.descriptor(IPCBuffer::from_ref(unk0, 0x15))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_product_model(&self, ) -> Result<i32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(79)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_ldn_channel(&self, ) -> Result<i32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(80)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_ldn_channel(&self, unk0: i32) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(81)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn acquire_telemetry_dirty_flag_event_handle(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(82)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn get_telemetry_dirty_flags(&self, ) -> Result<::ipcdefs::nn::settings::system::TelemetryDirtyFlag> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(83)
			.args(())
			;
		let res : Response<::ipcdefs::nn::settings::system::TelemetryDirtyFlag> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_ptm_battery_lot(&self, ) -> Result<::ipcdefs::nn::settings::factory::BatteryLot> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(84)
			.args(())
			;
		let res : Response<::ipcdefs::nn::settings::factory::BatteryLot> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_ptm_battery_lot(&self, unk0: ::ipcdefs::nn::settings::factory::BatteryLot) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(85)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_ptm_fuel_gauge_parameter(&self, ) -> Result<::ipcdefs::nn::settings::system::PtmFuelGaugeParameter> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(86)
			.args(())
			;
		let res : Response<::ipcdefs::nn::settings::system::PtmFuelGaugeParameter> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_ptm_fuel_gauge_parameter(&self, unk0: ::ipcdefs::nn::settings::system::PtmFuelGaugeParameter) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(87)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_bluetooth_enable_flag(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(88)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_bluetooth_enable_flag(&self, unk0: bool) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(89)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_mii_author_id(&self, ) -> Result<::ipcdefs::nn::util::Uuid> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(90)
			.args(())
			;
		let res : Response<::ipcdefs::nn::util::Uuid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_shutdown_rtc_value(&self, unk0: i64) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(91)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_shutdown_rtc_value(&self, ) -> Result<i64> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(92)
			.args(())
			;
		let res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn acquire_fatal_dirty_flag_event_handle(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(93)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn get_fatal_dirty_flags(&self, ) -> Result<::ipcdefs::nn::settings::system::FatalDirtyFlag> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(94)
			.args(())
			;
		let res : Response<::ipcdefs::nn::settings::system::FatalDirtyFlag> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn get_auto_update_enable_flag(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(95)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn set_auto_update_enable_flag(&self, unk0: bool) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(96)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn get_nx_controller_settings(&self, unk1: &mut [::ipcdefs::nn::settings::system::NxControllerSettings]) -> Result<i32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(97)
			.args(())
			.descriptor(IPCBuffer::from_mut_slice(unk1, 6))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn set_nx_controller_settings(&self, unk0: &[::ipcdefs::nn::settings::system::NxControllerSettings]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(98)
			.args(())
			.descriptor(IPCBuffer::from_slice(unk0, 5))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn get_battery_percentage_flag(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(99)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn set_battery_percentage_flag(&self, unk0: bool) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(100)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn get_external_rtc_reset_flag(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(101)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn set_external_rtc_reset_flag(&self, unk0: bool) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(102)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn get_usb_full_key_enable_flag(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(103)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn set_usb_full_key_enable_flag(&self, unk0: bool) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(104)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn set_external_steady_clock_internal_offset(&self, unk0: i64) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(105)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn get_external_steady_clock_internal_offset(&self, ) -> Result<i64> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(106)
			.args(())
			;
		let res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn get_backlight_settings_ex(&self, ) -> Result<::ipcdefs::nn::settings::system::BacklightSettingsEx> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(107)
			.args(())
			;
		let res : Response<::ipcdefs::nn::settings::system::BacklightSettingsEx> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn set_backlight_settings_ex(&self, unk0: ::ipcdefs::nn::settings::system::BacklightSettingsEx) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(108)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn get_headphone_volume_warning_count(&self, ) -> Result<i32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(109)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn set_headphone_volume_warning_count(&self, unk0: i32) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(110)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn get_bluetooth_afh_enable_flag(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(111)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn set_bluetooth_afh_enable_flag(&self, unk0: bool) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(112)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn get_bluetooth_boost_enable_flag(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(113)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn set_bluetooth_boost_enable_flag(&self, unk0: bool) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(114)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn get_in_repair_process_enable_flag(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(115)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn set_in_repair_process_enable_flag(&self, unk0: bool) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(116)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn get_headphone_volume_update_flag(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(117)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn set_headphone_volume_update_flag(&self, unk0: bool) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(118)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn needs_to_update_headphone_volume(&self, unk0: bool) -> Result<(bool, bool, i8)> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(119)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: bool,
			unk2: bool,
			unk3: i8,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().unk2.clone(),res.get_raw().unk3.clone()))
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn get_push_notification_activity_mode_on_sleep(&self, ) -> Result<i32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(120)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn set_push_notification_activity_mode_on_sleep(&self, unk0: i32) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(121)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_service_discovery_control_settings(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_service_discovery_control_settings(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_error_report_share_permission(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_error_report_share_permission(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_applet_launch_flags(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_applet_launch_flags(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_console_six_axis_sensor_acceleration_bias(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_console_six_axis_sensor_acceleration_bias(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_console_six_axis_sensor_angular_velocity_bias(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_console_six_axis_sensor_angular_velocity_bias(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_console_six_axis_sensor_acceleration_gain(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_console_six_axis_sensor_acceleration_gain(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_console_six_axis_sensor_angular_velocity_gain(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_console_six_axis_sensor_angular_velocity_gain(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_keyboard_layout(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_keyboard_layout(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_web_inspector_flag(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_allowed_ssl_hosts(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_host_fs_mount_point(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_requires_run_repair_time_reviser(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_requires_run_repair_time_reviser(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_ble_pairing_settings(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_ble_pairing_settings(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_console_six_axis_sensor_angular_velocity_time_bias(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_console_six_axis_sensor_angular_velocity_time_bias(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_console_six_axis_sensor_angular_acceleration(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_console_six_axis_sensor_angular_acceleration(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_rebootless_system_update_version(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for ISystemSettingsServer<T> {
	fn from(obj: T) -> ISystemSettingsServer<T> {
		ISystemSettingsServer(obj)
	}
}
