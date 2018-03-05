
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct ISystemSettingsServer(Session);

impl ISystemSettingsServer {
	pub fn SetLanguageCode(&self, unk0: ::nn::settings::LanguageCode) -> Result<()> {
		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn SetNetworkSettings(&self, unk0: &[::nn::settings::system::NetworkSettings]) -> Result<()> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetNetworkSettings(&self, unk1: &mut [::nn::settings::system::NetworkSettings]) -> Result<i32> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetFirmwareVersion(&self, unk0: &mut Option<::nn::settings::system::FirmwareVersion>) -> Result<()> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	#[cfg(feature = "switch-3.0.0")]
	pub fn GetFirmwareVersion2(&self, unk0: &mut Option<::nn::settings::system::FirmwareVersion>) -> Result<()> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetLockScreenFlag(&self, ) -> Result<bool> {
		let req = Request::new(7)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetLockScreenFlag(&self, unk0: bool) -> Result<()> {
		let req = Request::new(8)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetBacklightSettings(&self, ) -> Result<::nn::settings::system::BacklightSettings> {
		let req = Request::new(9)
			.args(())
			;
		let mut res : Response<::nn::settings::system::BacklightSettings> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetBacklightSettings(&self, unk0: ::nn::settings::system::BacklightSettings) -> Result<()> {
		let req = Request::new(10)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn SetBluetoothDevicesSettings(&self, unk0: &[::nn::settings::system::BluetoothDevicesSettings]) -> Result<()> {
		let req = Request::new(11)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetBluetoothDevicesSettings(&self, unk1: &mut [::nn::settings::system::BluetoothDevicesSettings]) -> Result<i32> {
		let req = Request::new(12)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetExternalSteadyClockSourceId(&self, ) -> Result<::nn::util::Uuid> {
		let req = Request::new(13)
			.args(())
			;
		let mut res : Response<::nn::util::Uuid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetExternalSteadyClockSourceId(&self, unk0: ::nn::util::Uuid) -> Result<()> {
		let req = Request::new(14)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetUserSystemClockContext(&self, ) -> Result<::nn::time::SystemClockContext> {
		let req = Request::new(15)
			.args(())
			;
		let mut res : Response<::nn::time::SystemClockContext> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetUserSystemClockContext(&self, unk0: ::nn::time::SystemClockContext) -> Result<()> {
		let req = Request::new(16)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetAccountSettings(&self, ) -> Result<::nn::settings::system::AccountSettings> {
		let req = Request::new(17)
			.args(())
			;
		let mut res : Response<::nn::settings::system::AccountSettings> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetAccountSettings(&self, unk0: ::nn::settings::system::AccountSettings) -> Result<()> {
		let req = Request::new(18)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetAudioVolume(&self, unk0: i32) -> Result<::nn::settings::system::AudioVolume> {
		let req = Request::new(19)
			.args(unk0)
			;
		let mut res : Response<::nn::settings::system::AudioVolume> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetAudioVolume(&self, unk0: ::nn::settings::system::AudioVolume, unk1: i32) -> Result<()> {
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
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetEulaVersions(&self, unk1: &mut [::nn::settings::system::EulaVersion]) -> Result<i32> {
		let req = Request::new(21)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetEulaVersions(&self, unk0: &[::nn::settings::system::EulaVersion]) -> Result<()> {
		let req = Request::new(22)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetColorSetId(&self, ) -> Result<i32> {
		let req = Request::new(23)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetColorSetId(&self, unk0: i32) -> Result<()> {
		let req = Request::new(24)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetConsoleInformationUploadFlag(&self, ) -> Result<bool> {
		let req = Request::new(25)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetConsoleInformationUploadFlag(&self, unk0: bool) -> Result<()> {
		let req = Request::new(26)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetAutomaticApplicationDownloadFlag(&self, ) -> Result<bool> {
		let req = Request::new(27)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetAutomaticApplicationDownloadFlag(&self, unk0: bool) -> Result<()> {
		let req = Request::new(28)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetNotificationSettings(&self, ) -> Result<::nn::settings::system::NotificationSettings> {
		let req = Request::new(29)
			.args(())
			;
		let mut res : Response<::nn::settings::system::NotificationSettings> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetNotificationSettings(&self, unk0: ::nn::settings::system::NotificationSettings) -> Result<()> {
		let req = Request::new(30)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetAccountNotificationSettings(&self, unk1: &mut [::nn::settings::system::AccountNotificationSettings]) -> Result<i32> {
		let req = Request::new(31)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetAccountNotificationSettings(&self, unk0: &[::nn::settings::system::AccountNotificationSettings]) -> Result<()> {
		let req = Request::new(32)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetVibrationMasterVolume(&self, ) -> Result<f32> {
		let req = Request::new(35)
			.args(())
			;
		let mut res : Response<f32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetVibrationMasterVolume(&self, unk0: f32) -> Result<()> {
		let req = Request::new(36)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetSettingsItemValueSize(&self, unk0: &::nn::settings::SettingsName, unk1: &::nn::settings::SettingsItemKey) -> Result<u64> {
		let req = Request::new(37)
			.args(())
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	// fn GetSettingsItemValue(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetTvSettings(&self, ) -> Result<::nn::settings::system::TvSettings> {
		let req = Request::new(39)
			.args(())
			;
		let mut res : Response<::nn::settings::system::TvSettings> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetTvSettings(&self, unk0: ::nn::settings::system::TvSettings) -> Result<()> {
		let req = Request::new(40)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetEdid(&self, unk0: &mut Option<::nn::settings::system::Edid>) -> Result<()> {
		let req = Request::new(41)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn SetEdid(&self, unk0: &::nn::settings::system::Edid) -> Result<()> {
		let req = Request::new(42)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetAudioOutputMode(&self, unk0: i32) -> Result<i32> {
		let req = Request::new(43)
			.args(unk0)
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetAudioOutputMode(&self, unk0: i32, unk1: i32) -> Result<()> {
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
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn IsForceMuteOnHeadphoneRemoved(&self, ) -> Result<bool> {
		let req = Request::new(45)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetForceMuteOnHeadphoneRemoved(&self, unk0: bool) -> Result<()> {
		let req = Request::new(46)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetQuestFlag(&self, ) -> Result<bool> {
		let req = Request::new(47)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetQuestFlag(&self, unk0: bool) -> Result<()> {
		let req = Request::new(48)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetDataDeletionSettings(&self, ) -> Result<::nn::settings::system::DataDeletionSettings> {
		let req = Request::new(49)
			.args(())
			;
		let mut res : Response<::nn::settings::system::DataDeletionSettings> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetDataDeletionSettings(&self, unk0: ::nn::settings::system::DataDeletionSettings) -> Result<()> {
		let req = Request::new(50)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetInitialSystemAppletProgramId(&self, ) -> Result<::nn::ncm::ProgramId> {
		let req = Request::new(51)
			.args(())
			;
		let mut res : Response<::nn::ncm::ProgramId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetOverlayDispProgramId(&self, ) -> Result<::nn::ncm::ProgramId> {
		let req = Request::new(52)
			.args(())
			;
		let mut res : Response<::nn::ncm::ProgramId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetDeviceTimeZoneLocationName(&self, ) -> Result<::nn::time::LocationName> {
		let req = Request::new(53)
			.args(())
			;
		let mut res : Response<::nn::time::LocationName> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetDeviceTimeZoneLocationName(&self, unk0: ::nn::time::LocationName) -> Result<()> {
		let req = Request::new(54)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetWirelessCertificationFileSize(&self, ) -> Result<u64> {
		let req = Request::new(55)
			.args(())
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	// fn GetWirelessCertificationFile(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn SetRegionCode(&self, unk0: i32) -> Result<()> {
		let req = Request::new(57)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetNetworkSystemClockContext(&self, ) -> Result<::nn::time::SystemClockContext> {
		let req = Request::new(58)
			.args(())
			;
		let mut res : Response<::nn::time::SystemClockContext> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetNetworkSystemClockContext(&self, unk0: ::nn::time::SystemClockContext) -> Result<()> {
		let req = Request::new(59)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn IsUserSystemClockAutomaticCorrectionEnabled(&self, ) -> Result<bool> {
		let req = Request::new(60)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetUserSystemClockAutomaticCorrectionEnabled(&self, unk0: bool) -> Result<()> {
		let req = Request::new(61)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetDebugModeFlag(&self, ) -> Result<bool> {
		let req = Request::new(62)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetPrimaryAlbumStorage(&self, ) -> Result<i32> {
		let req = Request::new(63)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetPrimaryAlbumStorage(&self, unk0: i32) -> Result<()> {
		let req = Request::new(64)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetUsb30EnableFlag(&self, ) -> Result<bool> {
		let req = Request::new(65)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetUsb30EnableFlag(&self, unk0: bool) -> Result<()> {
		let req = Request::new(66)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetBatteryLot(&self, ) -> Result<::nn::settings::system::BatteryLot> {
		let req = Request::new(67)
			.args(())
			;
		let mut res : Response<::nn::settings::system::BatteryLot> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetSerialNumber(&self, ) -> Result<::nn::settings::system::SerialNumber> {
		let req = Request::new(68)
			.args(())
			;
		let mut res : Response<::nn::settings::system::SerialNumber> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetNfcEnableFlag(&self, ) -> Result<bool> {
		let req = Request::new(69)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetNfcEnableFlag(&self, unk0: bool) -> Result<()> {
		let req = Request::new(70)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetSleepSettings(&self, ) -> Result<::nn::settings::system::SleepSettings> {
		let req = Request::new(71)
			.args(())
			;
		let mut res : Response<::nn::settings::system::SleepSettings> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetSleepSettings(&self, unk0: ::nn::settings::system::SleepSettings) -> Result<()> {
		let req = Request::new(72)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetWirelessLanEnableFlag(&self, ) -> Result<bool> {
		let req = Request::new(73)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetWirelessLanEnableFlag(&self, unk0: bool) -> Result<()> {
		let req = Request::new(74)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetInitialLaunchSettings(&self, ) -> Result<::nn::settings::system::InitialLaunchSettings> {
		let req = Request::new(75)
			.args(())
			;
		let mut res : Response<::nn::settings::system::InitialLaunchSettings> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetInitialLaunchSettings(&self, unk0: ::nn::settings::system::InitialLaunchSettings) -> Result<()> {
		let req = Request::new(76)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetDeviceNickName(&self, unk0: &mut Option<::nn::settings::system::DeviceNickName>) -> Result<()> {
		let req = Request::new(77)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn SetDeviceNickName(&self, unk0: &::nn::settings::system::DeviceNickName) -> Result<()> {
		let req = Request::new(78)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetProductModel(&self, ) -> Result<i32> {
		let req = Request::new(79)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetLdnChannel(&self, ) -> Result<i32> {
		let req = Request::new(80)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetLdnChannel(&self, unk0: i32) -> Result<()> {
		let req = Request::new(81)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn AcquireTelemetryDirtyFlagEventHandle(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetTelemetryDirtyFlags(&self, ) -> Result<::nn::settings::system::TelemetryDirtyFlag> {
		let req = Request::new(83)
			.args(())
			;
		let mut res : Response<::nn::settings::system::TelemetryDirtyFlag> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetPtmBatteryLot(&self, ) -> Result<::nn::settings::factory::BatteryLot> {
		let req = Request::new(84)
			.args(())
			;
		let mut res : Response<::nn::settings::factory::BatteryLot> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetPtmBatteryLot(&self, unk0: ::nn::settings::factory::BatteryLot) -> Result<()> {
		let req = Request::new(85)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetPtmFuelGaugeParameter(&self, ) -> Result<::nn::settings::system::PtmFuelGaugeParameter> {
		let req = Request::new(86)
			.args(())
			;
		let mut res : Response<::nn::settings::system::PtmFuelGaugeParameter> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetPtmFuelGaugeParameter(&self, unk0: ::nn::settings::system::PtmFuelGaugeParameter) -> Result<()> {
		let req = Request::new(87)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetBluetoothEnableFlag(&self, ) -> Result<bool> {
		let req = Request::new(88)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetBluetoothEnableFlag(&self, unk0: bool) -> Result<()> {
		let req = Request::new(89)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetMiiAuthorId(&self, ) -> Result<::nn::util::Uuid> {
		let req = Request::new(90)
			.args(())
			;
		let mut res : Response<::nn::util::Uuid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetShutdownRtcValue(&self, unk0: i64) -> Result<()> {
		let req = Request::new(91)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetShutdownRtcValue(&self, ) -> Result<i64> {
		let req = Request::new(92)
			.args(())
			;
		let mut res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	// fn AcquireFatalDirtyFlagEventHandle(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetFatalDirtyFlags(&self, ) -> Result<::nn::settings::system::FatalDirtyFlag> {
		let req = Request::new(94)
			.args(())
			;
		let mut res : Response<::nn::settings::system::FatalDirtyFlag> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	#[cfg(feature = "switch-2.0.0")]
	pub fn GetAutoUpdateEnableFlag(&self, ) -> Result<bool> {
		let req = Request::new(95)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	#[cfg(feature = "switch-2.0.0")]
	pub fn SetAutoUpdateEnableFlag(&self, unk0: bool) -> Result<()> {
		let req = Request::new(96)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	#[cfg(feature = "switch-2.0.0")]
	pub fn GetNxControllerSettings(&self, unk1: &mut [::nn::settings::system::NxControllerSettings]) -> Result<i32> {
		let req = Request::new(97)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	#[cfg(feature = "switch-2.0.0")]
	pub fn SetNxControllerSettings(&self, unk0: &[::nn::settings::system::NxControllerSettings]) -> Result<()> {
		let req = Request::new(98)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	#[cfg(feature = "switch-2.0.0")]
	pub fn GetBatteryPercentageFlag(&self, ) -> Result<bool> {
		let req = Request::new(99)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	#[cfg(feature = "switch-2.0.0")]
	pub fn SetBatteryPercentageFlag(&self, unk0: bool) -> Result<()> {
		let req = Request::new(100)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	#[cfg(feature = "switch-2.0.0")]
	pub fn GetExternalRtcResetFlag(&self, ) -> Result<bool> {
		let req = Request::new(101)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	#[cfg(feature = "switch-2.0.0")]
	pub fn SetExternalRtcResetFlag(&self, unk0: bool) -> Result<()> {
		let req = Request::new(102)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	#[cfg(feature = "switch-3.0.0")]
	pub fn GetUsbFullKeyEnableFlag(&self, ) -> Result<bool> {
		let req = Request::new(103)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	#[cfg(feature = "switch-3.0.0")]
	pub fn SetUsbFullKeyEnableFlag(&self, unk0: bool) -> Result<()> {
		let req = Request::new(104)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	#[cfg(feature = "switch-3.0.0")]
	pub fn SetExternalSteadyClockInternalOffset(&self, unk0: i64) -> Result<()> {
		let req = Request::new(105)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	#[cfg(feature = "switch-3.0.0")]
	pub fn GetExternalSteadyClockInternalOffset(&self, ) -> Result<i64> {
		let req = Request::new(106)
			.args(())
			;
		let mut res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	#[cfg(feature = "switch-3.0.0")]
	pub fn GetBacklightSettingsEx(&self, ) -> Result<::nn::settings::system::BacklightSettingsEx> {
		let req = Request::new(107)
			.args(())
			;
		let mut res : Response<::nn::settings::system::BacklightSettingsEx> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	#[cfg(feature = "switch-3.0.0")]
	pub fn SetBacklightSettingsEx(&self, unk0: ::nn::settings::system::BacklightSettingsEx) -> Result<()> {
		let req = Request::new(108)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	#[cfg(feature = "switch-3.0.0")]
	pub fn GetHeadphoneVolumeWarningCount(&self, ) -> Result<i32> {
		let req = Request::new(109)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	#[cfg(feature = "switch-3.0.0")]
	pub fn SetHeadphoneVolumeWarningCount(&self, unk0: i32) -> Result<()> {
		let req = Request::new(110)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	#[cfg(feature = "switch-3.0.0")]
	pub fn GetBluetoothAfhEnableFlag(&self, ) -> Result<bool> {
		let req = Request::new(111)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	#[cfg(feature = "switch-3.0.0")]
	pub fn SetBluetoothAfhEnableFlag(&self, unk0: bool) -> Result<()> {
		let req = Request::new(112)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	#[cfg(feature = "switch-3.0.0")]
	pub fn GetBluetoothBoostEnableFlag(&self, ) -> Result<bool> {
		let req = Request::new(113)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	#[cfg(feature = "switch-3.0.0")]
	pub fn SetBluetoothBoostEnableFlag(&self, unk0: bool) -> Result<()> {
		let req = Request::new(114)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	#[cfg(feature = "switch-3.0.0")]
	pub fn GetInRepairProcessEnableFlag(&self, ) -> Result<bool> {
		let req = Request::new(115)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	#[cfg(feature = "switch-3.0.0")]
	pub fn SetInRepairProcessEnableFlag(&self, unk0: bool) -> Result<()> {
		let req = Request::new(116)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	#[cfg(feature = "switch-3.0.0")]
	pub fn GetHeadphoneVolumeUpdateFlag(&self, ) -> Result<bool> {
		let req = Request::new(117)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	#[cfg(feature = "switch-3.0.0")]
	pub fn SetHeadphoneVolumeUpdateFlag(&self, unk0: bool) -> Result<()> {
		let req = Request::new(118)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	#[cfg(feature = "switch-3.0.0")]
	pub fn NeedsToUpdateHeadphoneVolume(&self, unk0: bool) -> Result<(bool, bool, i8)> {
		let req = Request::new(119)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: bool,
			unk2: bool,
			unk3: i8,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().unk2.clone(),res.get_raw().unk3.clone()))
	}
	#[cfg(feature = "switch-3.0.0")]
	pub fn GetPushNotificationActivityModeOnSleep(&self, ) -> Result<i32> {
		let req = Request::new(120)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	#[cfg(feature = "switch-3.0.0")]
	pub fn SetPushNotificationActivityModeOnSleep(&self, unk0: i32) -> Result<()> {
		let req = Request::new(121)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	#[cfg(feature = "switch-4.0.0")]
	pub fn GetServiceDiscoveryControlSettings(&self, ) -> Result<()> {
		let req = Request::new(122)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	#[cfg(feature = "switch-4.0.0")]
	pub fn SetServiceDiscoveryControlSettings(&self, ) -> Result<()> {
		let req = Request::new(123)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	#[cfg(feature = "switch-4.0.0")]
	pub fn GetErrorReportSharePermission(&self, ) -> Result<()> {
		let req = Request::new(124)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	#[cfg(feature = "switch-4.0.0")]
	pub fn SetErrorReportSharePermission(&self, ) -> Result<()> {
		let req = Request::new(125)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	#[cfg(feature = "switch-4.0.0")]
	pub fn GetAppletLaunchFlags(&self, ) -> Result<()> {
		let req = Request::new(126)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	#[cfg(feature = "switch-4.0.0")]
	pub fn SetAppletLaunchFlags(&self, ) -> Result<()> {
		let req = Request::new(127)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	#[cfg(feature = "switch-4.0.0")]
	pub fn GetConsoleSixAxisSensorAccelerationBias(&self, ) -> Result<()> {
		let req = Request::new(128)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	#[cfg(feature = "switch-4.0.0")]
	pub fn SetConsoleSixAxisSensorAccelerationBias(&self, ) -> Result<()> {
		let req = Request::new(129)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	#[cfg(feature = "switch-4.0.0")]
	pub fn GetConsoleSixAxisSensorAngularVelocityBias(&self, ) -> Result<()> {
		let req = Request::new(130)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	#[cfg(feature = "switch-4.0.0")]
	pub fn SetConsoleSixAxisSensorAngularVelocityBias(&self, ) -> Result<()> {
		let req = Request::new(131)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	#[cfg(feature = "switch-4.0.0")]
	pub fn GetConsoleSixAxisSensorAccelerationGain(&self, ) -> Result<()> {
		let req = Request::new(132)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	#[cfg(feature = "switch-4.0.0")]
	pub fn SetConsoleSixAxisSensorAccelerationGain(&self, ) -> Result<()> {
		let req = Request::new(133)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	#[cfg(feature = "switch-4.0.0")]
	pub fn GetConsoleSixAxisSensorAngularVelocityGain(&self, ) -> Result<()> {
		let req = Request::new(134)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	#[cfg(feature = "switch-4.0.0")]
	pub fn SetConsoleSixAxisSensorAngularVelocityGain(&self, ) -> Result<()> {
		let req = Request::new(135)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	#[cfg(feature = "switch-4.0.0")]
	pub fn GetKeyboardLayout(&self, ) -> Result<()> {
		let req = Request::new(136)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	#[cfg(feature = "switch-4.0.0")]
	pub fn SetKeyboardLayout(&self, ) -> Result<()> {
		let req = Request::new(137)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	#[cfg(feature = "switch-4.0.0")]
	pub fn GetWebInspectorFlag(&self, ) -> Result<()> {
		let req = Request::new(138)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	#[cfg(feature = "switch-4.0.0")]
	pub fn GetAllowedSslHosts(&self, ) -> Result<()> {
		let req = Request::new(139)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	#[cfg(feature = "switch-4.0.0")]
	pub fn GetHostFsMountPoint(&self, ) -> Result<()> {
		let req = Request::new(140)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
}

impl FromKObject for ISystemSettingsServer {
	unsafe fn from_kobject(obj: KObject) -> ISystemSettingsServer {
		ISystemSettingsServer(Session::from_kobject(obj))
	}
}
