
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct ISystemSettingsServer(Session);

impl ISystemSettingsServer {
	pub fn new() -> Result<ISystemSettingsServer> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"set:sys\0").map(|s| unsafe { ISystemSettingsServer::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for ISystemSettingsServer {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ISystemSettingsServer {
	pub fn set_language_code(&self, unk0: ::nn::settings::LanguageCode) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn set_network_settings(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_network_settings(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_firmware_version(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_firmware_version2(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_lock_screen_flag(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(7)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_lock_screen_flag(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(8)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_backlight_settings(&self, ) -> Result<::nn::settings::system::BacklightSettings> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(9)
			.args(())
			;
		let res : Response<::nn::settings::system::BacklightSettings> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_backlight_settings(&self, unk0: ::nn::settings::system::BacklightSettings) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn set_bluetooth_devices_settings(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_bluetooth_devices_settings(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_external_steady_clock_source_id(&self, ) -> Result<::nn::util::Uuid> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(13)
			.args(())
			;
		let res : Response<::nn::util::Uuid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_external_steady_clock_source_id(&self, unk0: ::nn::util::Uuid) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(14)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_user_system_clock_context(&self, ) -> Result<::nn::time::SystemClockContext> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(15)
			.args(())
			;
		let res : Response<::nn::time::SystemClockContext> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_user_system_clock_context(&self, unk0: ::nn::time::SystemClockContext) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(16)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_account_settings(&self, ) -> Result<::nn::settings::system::AccountSettings> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(17)
			.args(())
			;
		let res : Response<::nn::settings::system::AccountSettings> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_account_settings(&self, unk0: ::nn::settings::system::AccountSettings) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(18)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_audio_volume(&self, unk0: i32) -> Result<::nn::settings::system::AudioVolume> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(19)
			.args(unk0)
			;
		let res : Response<::nn::settings::system::AudioVolume> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_audio_volume(&self, unk0: ::nn::settings::system::AudioVolume, unk1: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::settings::system::AudioVolume,
			unk1: i32,
		}
		let req = Request::new(20)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_eula_versions(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_eula_versions(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_color_set_id(&self, ) -> Result<i32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(23)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_color_set_id(&self, unk0: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(24)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_console_information_upload_flag(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(25)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_console_information_upload_flag(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(26)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_automatic_application_download_flag(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(27)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_automatic_application_download_flag(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(28)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_notification_settings(&self, ) -> Result<::nn::settings::system::NotificationSettings> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(29)
			.args(())
			;
		let res : Response<::nn::settings::system::NotificationSettings> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_notification_settings(&self, unk0: ::nn::settings::system::NotificationSettings) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(30)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_account_notification_settings(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_account_notification_settings(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_vibration_master_volume(&self, ) -> Result<f32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(35)
			.args(())
			;
		let res : Response<f32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_vibration_master_volume(&self, unk0: f32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(36)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_settings_item_value_size(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_settings_item_value(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_tv_settings(&self, ) -> Result<::nn::settings::system::TvSettings> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(39)
			.args(())
			;
		let res : Response<::nn::settings::system::TvSettings> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_tv_settings(&self, unk0: ::nn::settings::system::TvSettings) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(40)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_edid(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_edid(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_audio_output_mode(&self, unk0: i32) -> Result<i32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(43)
			.args(unk0)
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_audio_output_mode(&self, unk0: i32, unk1: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: i32,
		}
		let req = Request::new(44)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_force_mute_on_headphone_removed(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(45)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_force_mute_on_headphone_removed(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(46)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_quest_flag(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(47)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_quest_flag(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(48)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_data_deletion_settings(&self, ) -> Result<::nn::settings::system::DataDeletionSettings> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(49)
			.args(())
			;
		let res : Response<::nn::settings::system::DataDeletionSettings> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_data_deletion_settings(&self, unk0: ::nn::settings::system::DataDeletionSettings) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(50)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_initial_system_applet_program_id(&self, ) -> Result<::nn::ncm::ProgramId> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(51)
			.args(())
			;
		let res : Response<::nn::ncm::ProgramId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_overlay_disp_program_id(&self, ) -> Result<::nn::ncm::ProgramId> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(52)
			.args(())
			;
		let res : Response<::nn::ncm::ProgramId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_device_time_zone_location_name(&self, ) -> Result<::nn::time::LocationName> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(53)
			.args(())
			;
		let res : Response<::nn::time::LocationName> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_device_time_zone_location_name(&self, unk0: ::nn::time::LocationName) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(54)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_wireless_certification_file_size(&self, ) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(55)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_wireless_certification_file(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn set_region_code(&self, unk0: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(57)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_network_system_clock_context(&self, ) -> Result<::nn::time::SystemClockContext> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(58)
			.args(())
			;
		let res : Response<::nn::time::SystemClockContext> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_network_system_clock_context(&self, unk0: ::nn::time::SystemClockContext) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(59)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_user_system_clock_automatic_correction_enabled(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(60)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_user_system_clock_automatic_correction_enabled(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(61)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_debug_mode_flag(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(62)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_primary_album_storage(&self, ) -> Result<i32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(63)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_primary_album_storage(&self, unk0: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(64)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_usb30_enable_flag(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(65)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_usb30_enable_flag(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(66)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_battery_lot(&self, ) -> Result<::nn::settings::system::BatteryLot> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(67)
			.args(())
			;
		let res : Response<::nn::settings::system::BatteryLot> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_serial_number(&self, ) -> Result<::nn::settings::system::SerialNumber> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(68)
			.args(())
			;
		let res : Response<::nn::settings::system::SerialNumber> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_nfc_enable_flag(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(69)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_nfc_enable_flag(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(70)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_sleep_settings(&self, ) -> Result<::nn::settings::system::SleepSettings> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(71)
			.args(())
			;
		let res : Response<::nn::settings::system::SleepSettings> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_sleep_settings(&self, unk0: ::nn::settings::system::SleepSettings) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(72)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_wireless_lan_enable_flag(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(73)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_wireless_lan_enable_flag(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(74)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_initial_launch_settings(&self, ) -> Result<::nn::settings::system::InitialLaunchSettings> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(75)
			.args(())
			;
		let res : Response<::nn::settings::system::InitialLaunchSettings> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_initial_launch_settings(&self, unk0: ::nn::settings::system::InitialLaunchSettings) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(76)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_device_nick_name(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_device_nick_name(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_product_model(&self, ) -> Result<i32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(79)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_ldn_channel(&self, ) -> Result<i32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(80)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_ldn_channel(&self, unk0: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(81)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn acquire_telemetry_dirty_flag_event_handle(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(82)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn get_telemetry_dirty_flags(&self, ) -> Result<::nn::settings::system::TelemetryDirtyFlag> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(83)
			.args(())
			;
		let res : Response<::nn::settings::system::TelemetryDirtyFlag> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_ptm_battery_lot(&self, ) -> Result<::nn::settings::factory::BatteryLot> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(84)
			.args(())
			;
		let res : Response<::nn::settings::factory::BatteryLot> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_ptm_battery_lot(&self, unk0: ::nn::settings::factory::BatteryLot) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(85)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_ptm_fuel_gauge_parameter(&self, ) -> Result<::nn::settings::system::PtmFuelGaugeParameter> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(86)
			.args(())
			;
		let res : Response<::nn::settings::system::PtmFuelGaugeParameter> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_ptm_fuel_gauge_parameter(&self, unk0: ::nn::settings::system::PtmFuelGaugeParameter) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(87)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_bluetooth_enable_flag(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(88)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_bluetooth_enable_flag(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(89)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_mii_author_id(&self, ) -> Result<::nn::util::Uuid> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(90)
			.args(())
			;
		let res : Response<::nn::util::Uuid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_shutdown_rtc_value(&self, unk0: i64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(91)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_shutdown_rtc_value(&self, ) -> Result<i64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(92)
			.args(())
			;
		let res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn acquire_fatal_dirty_flag_event_handle(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(93)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn get_fatal_dirty_flags(&self, ) -> Result<::nn::settings::system::FatalDirtyFlag> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(94)
			.args(())
			;
		let res : Response<::nn::settings::system::FatalDirtyFlag> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn get_auto_update_enable_flag(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(95)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn set_auto_update_enable_flag(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(96)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_nx_controller_settings(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_nx_controller_settings(&self, UNKNOWN) -> Result<UNKNOWN>;
	#[cfg(feature = "switch-2.0.0")]
	pub fn get_battery_percentage_flag(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(99)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn set_battery_percentage_flag(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(100)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn get_external_rtc_reset_flag(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(101)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn set_external_rtc_reset_flag(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(102)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn get_usb_full_key_enable_flag(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(103)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn set_usb_full_key_enable_flag(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(104)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn set_external_steady_clock_internal_offset(&self, unk0: i64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(105)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn get_external_steady_clock_internal_offset(&self, ) -> Result<i64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(106)
			.args(())
			;
		let res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn get_backlight_settings_ex(&self, ) -> Result<::nn::settings::system::BacklightSettingsEx> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(107)
			.args(())
			;
		let res : Response<::nn::settings::system::BacklightSettingsEx> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn set_backlight_settings_ex(&self, unk0: ::nn::settings::system::BacklightSettingsEx) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(108)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn get_headphone_volume_warning_count(&self, ) -> Result<i32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(109)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn set_headphone_volume_warning_count(&self, unk0: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(110)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn get_bluetooth_afh_enable_flag(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(111)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn set_bluetooth_afh_enable_flag(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(112)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn get_bluetooth_boost_enable_flag(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(113)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn set_bluetooth_boost_enable_flag(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(114)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn get_in_repair_process_enable_flag(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(115)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn set_in_repair_process_enable_flag(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(116)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn get_headphone_volume_update_flag(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(117)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn set_headphone_volume_update_flag(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(118)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn needs_to_update_headphone_volume(&self, unk0: bool) -> Result<(bool, bool, i8)> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(119)
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
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(120)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn set_push_notification_activity_mode_on_sleep(&self, unk0: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(121)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn get_service_discovery_control_settings(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(122)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn set_service_discovery_control_settings(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(123)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn get_error_report_share_permission(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(124)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn set_error_report_share_permission(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(125)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn get_applet_launch_flags(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(126)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn set_applet_launch_flags(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(127)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn get_console_six_axis_sensor_acceleration_bias(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(128)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn set_console_six_axis_sensor_acceleration_bias(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(129)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn get_console_six_axis_sensor_angular_velocity_bias(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(130)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn set_console_six_axis_sensor_angular_velocity_bias(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(131)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn get_console_six_axis_sensor_acceleration_gain(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(132)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn set_console_six_axis_sensor_acceleration_gain(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(133)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn get_console_six_axis_sensor_angular_velocity_gain(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(134)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn set_console_six_axis_sensor_angular_velocity_gain(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(135)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn get_keyboard_layout(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(136)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn set_keyboard_layout(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(137)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn get_web_inspector_flag(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(138)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn get_allowed_ssl_hosts(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(139)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn get_host_fs_mount_point(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(140)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for ISystemSettingsServer {
	unsafe fn from_kobject(obj: KObject) -> ISystemSettingsServer {
		ISystemSettingsServer(Session::from_kobject(obj))
	}
}
