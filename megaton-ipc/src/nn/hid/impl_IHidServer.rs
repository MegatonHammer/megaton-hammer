
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hamer::ipc::IPCBuffer

#[derive(Debug)]
pub struct IHidServer(Session);

impl IHidServer {
	pub fn get_service() -> Result<IHidServer> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"hid\0\0\0\0\0").map(|s| unsafe { IHidServer::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IHidServer {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IHidServer {
	pub fn CreateAppletResource(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<::nn::hid::IAppletResource> {
		let req = Request::new(0)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn ActivateDebugPad(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		let req = Request::new(1)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ActivateTouchScreen(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		let req = Request::new(11)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ActivateMouse(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		let req = Request::new(21)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ActivateKeyboard(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		let req = Request::new(31)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn AcquireXpadIdEventHandle(&self, unk0: u64) -> Result<KObject> {
		let req = Request::new(40)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn ReleaseXpadIdEventHandle(&self, unk0: u64) -> Result<()> {
		let req = Request::new(41)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ActivateXpad(&self, unk0: ::nn::hid::BasicXpadId, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::hid::BasicXpadId,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(51)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn GetXpadIds(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn ActivateJoyXpad(&self, unk0: ::nn::hid::JoyXpadId) -> Result<()> {
		let req = Request::new(56)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetJoyXpadLifoHandle(&self, unk0: ::nn::hid::JoyXpadId) -> Result<KObject> {
		let req = Request::new(58)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn GetJoyXpadIds(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn ActivateSixAxisSensor(&self, unk0: ::nn::hid::BasicXpadId) -> Result<()> {
		let req = Request::new(60)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DeactivateSixAxisSensor(&self, unk0: ::nn::hid::BasicXpadId) -> Result<()> {
		let req = Request::new(61)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetSixAxisSensorLifoHandle(&self, unk0: ::nn::hid::BasicXpadId) -> Result<KObject> {
		let req = Request::new(62)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn ActivateJoySixAxisSensor(&self, unk0: ::nn::hid::JoyXpadId) -> Result<()> {
		let req = Request::new(63)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DeactivateJoySixAxisSensor(&self, unk0: ::nn::hid::JoyXpadId) -> Result<()> {
		let req = Request::new(64)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetJoySixAxisSensorLifoHandle(&self, unk0: ::nn::hid::JoyXpadId) -> Result<KObject> {
		let req = Request::new(65)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn StartSixAxisSensor(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::hid::SixAxisSensorHandle,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(66)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn StopSixAxisSensor(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::hid::SixAxisSensorHandle,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(67)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn IsSixAxisSensorFusionEnabled(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<bool> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::hid::SixAxisSensorHandle,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(68)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn EnableSixAxisSensorFusion(&self, unk0: bool, unk1: ::nn::hid::SixAxisSensorHandle, unk2: ::nn::applet::AppletResourceUserId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: ::nn::hid::SixAxisSensorHandle,
			unk2: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(69)
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

	pub fn SetSixAxisSensorFusionParameters(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: f32, unk2: f32, unk3: ::nn::applet::AppletResourceUserId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::hid::SixAxisSensorHandle,
			unk1: f32,
			unk2: f32,
			unk3: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(70)
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

	pub fn GetSixAxisSensorFusionParameters(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<(f32, f32)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::hid::SixAxisSensorHandle,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(71)
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

	pub fn ResetSixAxisSensorFusionParameters(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::hid::SixAxisSensorHandle,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(72)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetAccelerometerParameters(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: f32, unk2: f32, unk3: ::nn::applet::AppletResourceUserId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::hid::SixAxisSensorHandle,
			unk1: f32,
			unk2: f32,
			unk3: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(73)
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

	pub fn GetAccelerometerParameters(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<(f32, f32)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::hid::SixAxisSensorHandle,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(74)
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

	pub fn ResetAccelerometerParameters(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::hid::SixAxisSensorHandle,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(75)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetAccelerometerPlayMode(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: u32, unk2: ::nn::applet::AppletResourceUserId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::hid::SixAxisSensorHandle,
			unk1: u32,
			unk2: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(76)
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

	pub fn GetAccelerometerPlayMode(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<u32> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::hid::SixAxisSensorHandle,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(77)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn ResetAccelerometerPlayMode(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::hid::SixAxisSensorHandle,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(78)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetGyroscopeZeroDriftMode(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: u32, unk2: ::nn::applet::AppletResourceUserId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::hid::SixAxisSensorHandle,
			unk1: u32,
			unk2: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(79)
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

	pub fn GetGyroscopeZeroDriftMode(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<u32> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::hid::SixAxisSensorHandle,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(80)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn ResetGyroscopeZeroDriftMode(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::hid::SixAxisSensorHandle,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(81)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn IsSixAxisSensorAtRest(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<bool> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::hid::SixAxisSensorHandle,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(82)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn ActivateGesture(&self, unk0: i32, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(91)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetSupportedNpadStyleSet(&self, unk0: ::nn::hid::NpadStyleTag, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::hid::NpadStyleTag,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(100)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetSupportedNpadStyleSet(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<::nn::hid::NpadStyleTag> {
		let req = Request::new(101)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<::nn::hid::NpadStyleTag> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn SetSupportedNpadIdType(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn ActivateNpad(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		let req = Request::new(103)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DeactivateNpad(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		let req = Request::new(104)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn AcquireNpadStyleSetUpdateEventHandle(&self, unk0: u32, unk1: ::nn::applet::AppletResourceUserId, unk2: u64) -> Result<KObject> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: ::nn::applet::AppletResourceUserId,
			unk2: u64,
		}
		let req = Request::new(106)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn DisconnectNpad(&self, unk0: u32, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(107)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetPlayerLedPattern(&self, unk0: u32) -> Result<u64> {
		let req = Request::new(108)
			.args(unk0)
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SetNpadJoyHoldType(&self, unk0: ::nn::applet::AppletResourceUserId, unk1: i64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::applet::AppletResourceUserId,
			unk1: i64,
		}
		let req = Request::new(120)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetNpadJoyHoldType(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<i64> {
		let req = Request::new(121)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SetNpadJoyAssignmentModeSingleByDefault(&self, unk0: u32, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(122)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetNpadJoyAssignmentModeSingle(&self, unk0: u32, unk1: ::nn::applet::AppletResourceUserId, unk2: i64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: ::nn::applet::AppletResourceUserId,
			unk2: i64,
		}
		let req = Request::new(123)
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

	pub fn SetNpadJoyAssignmentModeDual(&self, unk0: u32, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(124)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn MergeSingleJoyAsDualJoy(&self, unk0: u32, unk1: u32, unk2: ::nn::applet::AppletResourceUserId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u32,
			unk2: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(125)
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

	pub fn StartLrAssignmentMode(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		let req = Request::new(126)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn StopLrAssignmentMode(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		let req = Request::new(127)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetNpadHandheldActivationMode(&self, unk0: ::nn::applet::AppletResourceUserId, unk1: i64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::applet::AppletResourceUserId,
			unk1: i64,
		}
		let req = Request::new(128)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetNpadHandheldActivationMode(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<i64> {
		let req = Request::new(129)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SwapNpadAssignment(&self, unk0: u32, unk1: u32, unk2: ::nn::applet::AppletResourceUserId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u32,
			unk2: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(130)
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

	pub fn IsUnintendedHomeButtonInputProtectionEnabled(&self, unk0: u32, unk1: ::nn::applet::AppletResourceUserId) -> Result<bool> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(131)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn EnableUnintendedHomeButtonInputProtection(&self, unk0: bool, unk1: u32, unk2: ::nn::applet::AppletResourceUserId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: u32,
			unk2: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(132)
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

	pub fn GetVibrationDeviceInfo(&self, unk0: ::nn::hid::VibrationDeviceHandle) -> Result<::nn::hid::VibrationDeviceInfoForIpc> {
		let req = Request::new(200)
			.args(unk0)
			;
		let mut res : Response<::nn::hid::VibrationDeviceInfoForIpc> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn SendVibrationValue(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetActualVibrationValue(&self, unk0: ::nn::hid::VibrationDeviceHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<::nn::hid::VibrationValue> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::hid::VibrationDeviceHandle,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(202)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<::nn::hid::VibrationValue> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn CreateActiveVibrationDeviceList(&self, ) -> Result<::nn::hid::IActiveVibrationDeviceList> {
		let req = Request::new(203)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn PermitVibration(&self, unk0: bool) -> Result<()> {
		let req = Request::new(204)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn IsVibrationPermitted(&self, ) -> Result<bool> {
		let req = Request::new(205)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn SendVibrationValues(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn ActivateConsoleSixAxisSensor(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		let req = Request::new(300)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn StartConsoleSixAxisSensor(&self, unk0: ::nn::hid::ConsoleSixAxisSensorHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::hid::ConsoleSixAxisSensorHandle,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(301)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn StopConsoleSixAxisSensor(&self, unk0: ::nn::hid::ConsoleSixAxisSensorHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::hid::ConsoleSixAxisSensorHandle,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(302)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn IsUsbFullKeyControllerEnabled(&self, ) -> Result<bool> {
		let req = Request::new(400)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn EnableUsbFullKeyController(&self, unk0: bool) -> Result<()> {
		let req = Request::new(401)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn IsUsbFullKeyControllerConnected(&self, unk0: u32) -> Result<bool> {
		let req = Request::new(402)
			.args(unk0)
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SetNpadCommunicationMode(&self, unk0: ::nn::applet::AppletResourceUserId, unk1: i64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::applet::AppletResourceUserId,
			unk1: i64,
		}
		let req = Request::new(1000)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetNpadCommunicationMode(&self, ) -> Result<i64> {
		let req = Request::new(1001)
			.args(())
			;
		let mut res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IHidServer {
	unsafe fn from_kobject(obj: KObject) -> IHidServer {
		IHidServer(Session::from_kobject(obj))
	}
}
