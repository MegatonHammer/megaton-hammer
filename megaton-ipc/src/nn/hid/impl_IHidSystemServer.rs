
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IHidSystemServer(Session);

impl IHidSystemServer {
	pub fn get_service() -> Result<IHidSystemServer> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"hid:sys\0").map(|s| unsafe { IHidSystemServer::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IHidSystemServer {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IHidSystemServer {
	pub fn SendKeyboardLockKeyEvent(&self, unk0: ::nn::hid::system::KeyboardLockKeyEvent) -> Result<()> {
		let req = Request::new(31)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn AcquireHomeButtonEventHandle(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<KObject> {
		let req = Request::new(101)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn ActivateHomeButton(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		let req = Request::new(111)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn AcquireSleepButtonEventHandle(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<KObject> {
		let req = Request::new(121)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn ActivateSleepButton(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		let req = Request::new(131)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn AcquireCaptureButtonEventHandle(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<KObject> {
		let req = Request::new(141)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn ActivateCaptureButton(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		let req = Request::new(151)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn AcquireNfcDeviceUpdateEventHandle(&self, ) -> Result<KObject> {
		let req = Request::new(210)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn GetNpadsWithNfc(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn AcquireNfcActivateEventHandle(&self, unk0: u32) -> Result<KObject> {
		let req = Request::new(212)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn ActivateNfc(&self, unk0: bool, unk1: u32, unk2: ::nn::applet::AppletResourceUserId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: u32,
			unk2: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(213)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn AcquireIrSensorEventHandle(&self, unk0: u32) -> Result<KObject> {
		let req = Request::new(230)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn ActivateIrSensor(&self, unk0: bool, unk1: u32, unk2: ::nn::applet::AppletResourceUserId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: u32,
			unk2: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(231)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ActivateNpadSystem(&self, unk0: u32) -> Result<()> {
		let req = Request::new(301)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ApplyNpadSystemCommonPolicy(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		let req = Request::new(303)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn EnableAssigningSingleOnSlSrPress(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		let req = Request::new(304)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DisableAssigningSingleOnSlSrPress(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		let req = Request::new(305)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetLastActiveNpad(&self, ) -> Result<u32> {
		let req = Request::new(306)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetNpadSystemExtStyle(&self, unk0: u32) -> Result<(i64, i64)> {
		let req = Request::new(307)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: i64,
			unk2: i64,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().unk2.clone()))
	}

	pub fn SetNpadPlayerLedBlinkingDevice(&self, unk0: u32, unk1: ::nn::hid::system::DeviceType, unk2: ::nn::applet::AppletResourceUserId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: ::nn::hid::system::DeviceType,
			unk2: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(311)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn GetUniquePadsFromNpad(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetIrSensorState(&self, unk0: u32, unk1: ::nn::applet::AppletResourceUserId) -> Result<i64> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(322)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetXcdHandleForNpadWithIrSensor(&self, unk0: u32, unk1: ::nn::applet::AppletResourceUserId) -> Result<u64> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(323)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SetAppletResourceUserId(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		let req = Request::new(500)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn RegisterAppletResourceUserId(&self, unk0: bool, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(501)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn UnregisterAppletResourceUserId(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		let req = Request::new(502)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn EnableAppletToGetInput(&self, unk0: bool, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(503)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetAruidValidForVibration(&self, unk0: bool, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(504)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn EnableAppletToGetSixAxisSensor(&self, unk0: bool, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(505)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetVibrationMasterVolume(&self, unk0: f32) -> Result<()> {
		let req = Request::new(510)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetVibrationMasterVolume(&self, ) -> Result<f32> {
		let req = Request::new(511)
			.args(())
			;
		let mut res : Response<f32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn BeginPermitVibrationSession(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		let req = Request::new(512)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn EndPermitVibrationSession(&self, ) -> Result<()> {
		let req = Request::new(513)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn EnableHandheldHids(&self, ) -> Result<()> {
		let req = Request::new(520)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DisableHandheldHids(&self, ) -> Result<()> {
		let req = Request::new(521)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn AcquirePlayReportControllerUsageUpdateEvent(&self, ) -> Result<KObject> {
		let req = Request::new(540)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn GetPlayReportControllerUsages(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn AcquirePlayReportRegisteredDeviceUpdateEvent(&self, ) -> Result<KObject> {
		let req = Request::new(542)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn GetRegisteredDevices(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn AcquireConnectionTriggerTimeoutEvent(&self, ) -> Result<KObject> {
		let req = Request::new(544)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn SendConnectionTrigger(&self, unk0: ::nn::bluetooth::Address) -> Result<()> {
		let req = Request::new(545)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn AcquireDeviceRegisteredEventForControllerSupport(&self, ) -> Result<KObject> {
		let req = Request::new(546)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn GetAllowedBluetoothLinksCount(&self, ) -> Result<i64> {
		let req = Request::new(547)
			.args(())
			;
		let mut res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn ActivateUniquePad(&self, unk0: ::nn::applet::AppletResourceUserId, unk1: ::nn::hid::system::UniquePadId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::applet::AppletResourceUserId,
			unk1: ::nn::hid::system::UniquePadId,
		}
		let req = Request::new(700)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn AcquireUniquePadConnectionEventHandle(&self, ) -> Result<KObject> {
		let req = Request::new(702)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn GetUniquePadIds(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn AcquireJoyDetachOnBluetoothOffEventHandle(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<KObject> {
		let req = Request::new(751)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn ListSixAxisSensorHandles(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn IsSixAxisSensorUserCalibrationSupported(&self, unk0: ::nn::hid::system::UniqueSixAxisSensorHandle) -> Result<bool> {
		let req = Request::new(801)
			.args(unk0)
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn ResetSixAxisSensorCalibrationValues(&self, unk0: ::nn::hid::system::UniqueSixAxisSensorHandle) -> Result<()> {
		let req = Request::new(802)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn StartSixAxisSensorUserCalibration(&self, unk0: ::nn::hid::system::UniqueSixAxisSensorHandle) -> Result<()> {
		let req = Request::new(803)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn CancelSixAxisSensorUserCalibration(&self, unk0: ::nn::hid::system::UniqueSixAxisSensorHandle) -> Result<()> {
		let req = Request::new(804)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetUniquePadBluetoothAddress(&self, unk0: ::nn::hid::system::UniquePadId) -> Result<::nn::bluetooth::Address> {
		let req = Request::new(805)
			.args(unk0)
			;
		let mut res : Response<::nn::bluetooth::Address> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn DisconnectUniquePad(&self, unk0: ::nn::hid::system::UniquePadId) -> Result<()> {
		let req = Request::new(806)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn StartAnalogStickManualCalibration(&self, unk0: ::nn::hid::system::UniquePadId, unk1: i64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::hid::system::UniquePadId,
			unk1: i64,
		}
		let req = Request::new(821)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn RetryCurrentAnalogStickManualCalibrationStage(&self, unk0: ::nn::hid::system::UniquePadId, unk1: i64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::hid::system::UniquePadId,
			unk1: i64,
		}
		let req = Request::new(822)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn CancelAnalogStickManualCalibration(&self, unk0: ::nn::hid::system::UniquePadId, unk1: i64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::hid::system::UniquePadId,
			unk1: i64,
		}
		let req = Request::new(823)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ResetAnalogStickManualCalibration(&self, unk0: ::nn::hid::system::UniquePadId, unk1: i64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::hid::system::UniquePadId,
			unk1: i64,
		}
		let req = Request::new(824)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn IsUsbFullKeyControllerEnabled(&self, ) -> Result<bool> {
		let req = Request::new(850)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn EnableUsbFullKeyController(&self, unk0: bool) -> Result<()> {
		let req = Request::new(851)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn IsUsbConnected(&self, unk0: ::nn::hid::system::UniquePadId) -> Result<bool> {
		let req = Request::new(852)
			.args(unk0)
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn ActivateInputDetector(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		let req = Request::new(900)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn NotifyInputDetector(&self, unk0: ::nn::hid::system::InputSourceId) -> Result<()> {
		let req = Request::new(901)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn InitializeFirmwareUpdate(&self, ) -> Result<()> {
		let req = Request::new(1000)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetFirmwareVersion(&self, unk0: ::nn::hid::system::UniquePadId) -> Result<::nn::hid::system::FirmwareVersion> {
		let req = Request::new(1001)
			.args(unk0)
			;
		let mut res : Response<::nn::hid::system::FirmwareVersion> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetAvailableFirmwareVersion(&self, unk0: ::nn::hid::system::UniquePadId) -> Result<::nn::hid::system::FirmwareVersion> {
		let req = Request::new(1002)
			.args(unk0)
			;
		let mut res : Response<::nn::hid::system::FirmwareVersion> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn IsFirmwareUpdateAvailable(&self, unk0: ::nn::hid::system::UniquePadId) -> Result<bool> {
		let req = Request::new(1003)
			.args(unk0)
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn CheckFirmwareUpdateRequired(&self, unk0: ::nn::hid::system::UniquePadId) -> Result<i64> {
		let req = Request::new(1004)
			.args(unk0)
			;
		let mut res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn StartFirmwareUpdate(&self, unk0: ::nn::hid::system::UniquePadId) -> Result<::nn::hid::system::FirmwareUpdateDeviceHandle> {
		let req = Request::new(1005)
			.args(unk0)
			;
		let mut res : Response<::nn::hid::system::FirmwareUpdateDeviceHandle> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn AbortFirmwareUpdate(&self, ) -> Result<()> {
		let req = Request::new(1006)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetFirmwareUpdateState(&self, unk0: ::nn::hid::system::FirmwareUpdateDeviceHandle) -> Result<::nn::hid::system::FirmwareUpdateState> {
		let req = Request::new(1007)
			.args(unk0)
			;
		let mut res : Response<::nn::hid::system::FirmwareUpdateState> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IHidSystemServer {
	unsafe fn from_kobject(obj: KObject) -> IHidSystemServer {
		IHidSystemServer(Session::from_kobject(obj))
	}
}
