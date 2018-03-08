
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IHidDebugServer(Session);

impl IHidDebugServer {
	pub fn DeactivateDebugPad(&self, ) -> Result<()> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetDebugPadAutoPilotState(&self, unk0: ::nn::hid::debug::DebugPadAutoPilotState) -> Result<()> {
		let req = Request::new(1)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn UnsetDebugPadAutoPilotState(&self, ) -> Result<()> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DeactivateTouchScreen(&self, ) -> Result<()> {
		let req = Request::new(10)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetTouchScreenAutoPilotState(&self, unk0: &[::nn::hid::TouchState]) -> Result<()> {
		let req = Request::new(11)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn UnsetTouchScreenAutoPilotState(&self, ) -> Result<()> {
		let req = Request::new(12)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DeactivateMouse(&self, ) -> Result<()> {
		let req = Request::new(20)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetMouseAutoPilotState(&self, unk0: ::nn::hid::debug::MouseAutoPilotState) -> Result<()> {
		let req = Request::new(21)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn UnsetMouseAutoPilotState(&self, ) -> Result<()> {
		let req = Request::new(22)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DeactivateKeyboard(&self, ) -> Result<()> {
		let req = Request::new(30)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetKeyboardAutoPilotState(&self, unk0: ::nn::hid::debug::KeyboardAutoPilotState) -> Result<()> {
		let req = Request::new(31)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn UnsetKeyboardAutoPilotState(&self, ) -> Result<()> {
		let req = Request::new(32)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DeactivateXpad(&self, unk0: ::nn::hid::BasicXpadId) -> Result<()> {
		let req = Request::new(50)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn SetXpadAutoPilotState(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn UnsetXpadAutoPilotState(&self, unk0: ::nn::hid::BasicXpadId) -> Result<()> {
		let req = Request::new(52)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DeactivateJoyXpad(&self, unk0: ::nn::hid::JoyXpadId) -> Result<()> {
		let req = Request::new(60)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DeactivateGesture(&self, ) -> Result<()> {
		let req = Request::new(91)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DeactivateHomeButton(&self, ) -> Result<()> {
		let req = Request::new(110)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetHomeButtonAutoPilotState(&self, unk0: ::nn::hid::debug::HomeButtonAutoPilotState) -> Result<()> {
		let req = Request::new(111)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn UnsetHomeButtonAutoPilotState(&self, ) -> Result<()> {
		let req = Request::new(112)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DeactivateSleepButton(&self, ) -> Result<()> {
		let req = Request::new(120)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetSleepButtonAutoPilotState(&self, unk0: ::nn::hid::debug::SleepButtonAutoPilotState) -> Result<()> {
		let req = Request::new(121)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn UnsetSleepButtonAutoPilotState(&self, ) -> Result<()> {
		let req = Request::new(122)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DeactivateInputDetector(&self, ) -> Result<()> {
		let req = Request::new(123)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DeactivateCaptureButton(&self, ) -> Result<()> {
		let req = Request::new(130)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetCaptureButtonAutoPilotState(&self, unk0: ::nn::hid::debug::CaptureButtonAutoPilotState) -> Result<()> {
		let req = Request::new(131)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn UnsetCaptureButtonAutoPilotState(&self, ) -> Result<()> {
		let req = Request::new(132)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetShiftAccelerometerCalibrationValue(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: f32, unk2: f32, unk3: ::nn::applet::AppletResourceUserId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::hid::SixAxisSensorHandle,
			unk1: f32,
			unk2: f32,
			unk3: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(133)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
				unk3,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetShiftAccelerometerCalibrationValue(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<(f32, f32)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::hid::SixAxisSensorHandle,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(134)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk3: f32,
			unk4: f32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk3.clone(),res.get_raw().unk4.clone()))
	}

	pub fn SetShiftGyroscopeCalibrationValue(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: f32, unk2: f32, unk3: ::nn::applet::AppletResourceUserId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::hid::SixAxisSensorHandle,
			unk1: f32,
			unk2: f32,
			unk3: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(135)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
				unk3,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetShiftGyroscopeCalibrationValue(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<(f32, f32)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::hid::SixAxisSensorHandle,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(136)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk3: f32,
			unk4: f32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk3.clone(),res.get_raw().unk4.clone()))
	}

	pub fn DeactivateConsoleSixAxisSensor(&self, ) -> Result<()> {
		let req = Request::new(140)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ActivateFirmwareUpdate(&self, ) -> Result<()> {
		let req = Request::new(201)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DeactivateFirmwareUpdate(&self, ) -> Result<()> {
		let req = Request::new(202)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn StartFirmwareUpdate(&self, unk0: ::nn::hid::system::UniquePadId) -> Result<()> {
		let req = Request::new(203)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetFirmwareUpdateStage(&self, ) -> Result<(i64, i64)> {
		let req = Request::new(204)
			.args(())
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk0: i64,
			unk1: i64,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk0.clone(),res.get_raw().unk1.clone()))
	}

	pub fn GetFirmwareVersion(&self, unk0: u32, unk1: ::nn::hid::system::DeviceType) -> Result<(::nn::hid::system::FirmwareVersion)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: ::nn::hid::system::DeviceType,
		}
		let req = Request::new(205)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<::nn::hid::system::FirmwareVersion> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetDestinationFirmwareVersion(&self, unk0: u32, unk1: ::nn::hid::system::DeviceType) -> Result<(::nn::hid::system::FirmwareVersion)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: ::nn::hid::system::DeviceType,
		}
		let req = Request::new(206)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<::nn::hid::system::FirmwareVersion> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn DiscardFirmwareInfoCacheForRevert(&self, ) -> Result<()> {
		let req = Request::new(207)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn StartFirmwareUpdateForRevert(&self, unk0: ::nn::hid::system::UniquePadId) -> Result<()> {
		let req = Request::new(208)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetAvailableFirmwareVersionForRevert(&self, unk0: ::nn::hid::system::UniquePadId) -> Result<(::nn::hid::system::FirmwareVersion)> {
		let req = Request::new(209)
			.args(unk0)
			;
		let mut res : Response<::nn::hid::system::FirmwareVersion> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn UpdateControllerColor(&self, ) -> Result<()> {
		let req = Request::new(211)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IHidDebugServer {
	unsafe fn from_kobject(obj: KObject) -> IHidDebugServer {
		IHidDebugServer(Session::from_kobject(obj))
	}
}
