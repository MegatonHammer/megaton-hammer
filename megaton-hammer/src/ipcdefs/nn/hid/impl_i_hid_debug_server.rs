
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::sync::Arc;

#[derive(Debug)]
pub struct IHidDebugServer<T>(T);

impl IHidDebugServer<Session> {
	pub fn raw_new() -> Result<IHidDebugServer<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"hid:dbg\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IHidDebugServer<Session>>> {
		use alloc::sync::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IHidDebugServer<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"hid:dbg\0") {
			let ret = Arc::new(IHidDebugServer(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IHidDebugServer<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IHidDebugServer(domain)),
			Err((sess, err)) => Err((IHidDebugServer(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IHidDebugServer<Session>> {
		Ok(IHidDebugServer(self.0.duplicate()?))
	}
}

impl<T> Deref for IHidDebugServer<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IHidDebugServer<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IHidDebugServer<T> {
	pub fn deactivate_debug_pad(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_debug_pad_auto_pilot_state(&self, unk0: ::ipcdefs::nn::hid::debug::DebugPadAutoPilotState) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unset_debug_pad_auto_pilot_state(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn deactivate_touch_screen(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(10)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_touch_screen_auto_pilot_state(&self, unk0: &[::ipcdefs::nn::hid::TouchState]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(11)
			.args(())
			.descriptor(IPCBuffer::from_slice(unk0, 5))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unset_touch_screen_auto_pilot_state(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(12)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn deactivate_mouse(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(20)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_mouse_auto_pilot_state(&self, unk0: ::ipcdefs::nn::hid::debug::MouseAutoPilotState) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(21)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unset_mouse_auto_pilot_state(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(22)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn deactivate_keyboard(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_keyboard_auto_pilot_state(&self, unk0: ::ipcdefs::nn::hid::debug::KeyboardAutoPilotState) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(31)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unset_keyboard_auto_pilot_state(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(32)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn deactivate_xpad(&self, unk0: ::ipcdefs::nn::hid::BasicXpadId) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(50)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn set_xpad_auto_pilot_state(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unset_xpad_auto_pilot_state(&self, unk0: ::ipcdefs::nn::hid::BasicXpadId) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(52)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn deactivate_joy_xpad(&self, unk0: ::ipcdefs::nn::hid::JoyXpadId) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(60)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn deactivate_gesture(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(91)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn deactivate_home_button(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(110)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_home_button_auto_pilot_state(&self, unk0: ::ipcdefs::nn::hid::debug::HomeButtonAutoPilotState) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(111)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unset_home_button_auto_pilot_state(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(112)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn deactivate_sleep_button(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(120)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_sleep_button_auto_pilot_state(&self, unk0: ::ipcdefs::nn::hid::debug::SleepButtonAutoPilotState) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(121)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unset_sleep_button_auto_pilot_state(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(122)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn deactivate_input_detector(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(123)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn deactivate_capture_button(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(130)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_capture_button_auto_pilot_state(&self, unk0: ::ipcdefs::nn::hid::debug::CaptureButtonAutoPilotState) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(131)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unset_capture_button_auto_pilot_state(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(132)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_shift_accelerometer_calibration_value(&self, unk0: ::ipcdefs::nn::hid::SixAxisSensorHandle, unk1: f32, unk2: f32, unk3: ::ipcdefs::nn::applet::AppletResourceUserId) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::hid::SixAxisSensorHandle,
			unk1: f32,
			unk2: f32,
			unk3: ::ipcdefs::nn::applet::AppletResourceUserId,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(133)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
				unk3,
			})
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_shift_accelerometer_calibration_value(&self, unk0: ::ipcdefs::nn::hid::SixAxisSensorHandle, unk1: ::ipcdefs::nn::applet::AppletResourceUserId) -> Result<(f32, f32)> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::hid::SixAxisSensorHandle,
			unk1: ::ipcdefs::nn::applet::AppletResourceUserId,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(134)
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
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk3.clone(),res.get_raw().unk4.clone()))
	}

	pub fn set_shift_gyroscope_calibration_value(&self, unk0: ::ipcdefs::nn::hid::SixAxisSensorHandle, unk1: f32, unk2: f32, unk3: ::ipcdefs::nn::applet::AppletResourceUserId) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::hid::SixAxisSensorHandle,
			unk1: f32,
			unk2: f32,
			unk3: ::ipcdefs::nn::applet::AppletResourceUserId,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(135)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
				unk3,
			})
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_shift_gyroscope_calibration_value(&self, unk0: ::ipcdefs::nn::hid::SixAxisSensorHandle, unk1: ::ipcdefs::nn::applet::AppletResourceUserId) -> Result<(f32, f32)> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::hid::SixAxisSensorHandle,
			unk1: ::ipcdefs::nn::applet::AppletResourceUserId,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(136)
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
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk3.clone(),res.get_raw().unk4.clone()))
	}

	pub fn deactivate_console_six_axis_sensor(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(140)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_console_six_axis_sensor_sampling_frequency(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn deactivate_seven_six_axis_sensor(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn activate_firmware_update(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(201)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn deactivate_firmware_update(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(202)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn start_firmware_update(&self, unk0: ::ipcdefs::nn::hid::system::UniquePadId) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(203)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_firmware_update_stage(&self, ) -> Result<(i64, i64)> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(204)
			.args(())
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk0: i64,
			unk1: i64,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk0.clone(),res.get_raw().unk1.clone()))
	}

	pub fn get_firmware_version(&self, unk0: u32, unk1: ::ipcdefs::nn::hid::system::DeviceType) -> Result<::ipcdefs::nn::hid::system::FirmwareVersion> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: ::ipcdefs::nn::hid::system::DeviceType,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(205)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let res : Response<::ipcdefs::nn::hid::system::FirmwareVersion> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_destination_firmware_version(&self, unk0: u32, unk1: ::ipcdefs::nn::hid::system::DeviceType) -> Result<::ipcdefs::nn::hid::system::FirmwareVersion> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: ::ipcdefs::nn::hid::system::DeviceType,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(206)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let res : Response<::ipcdefs::nn::hid::system::FirmwareVersion> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn discard_firmware_info_cache_for_revert(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(207)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn start_firmware_update_for_revert(&self, unk0: ::ipcdefs::nn::hid::system::UniquePadId) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(208)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_available_firmware_version_for_revert(&self, unk0: ::ipcdefs::nn::hid::system::UniquePadId) -> Result<::ipcdefs::nn::hid::system::FirmwareVersion> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(209)
			.args(unk0)
			;
		let res : Response<::ipcdefs::nn::hid::system::FirmwareVersion> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn is_firmware_updating_device(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn update_controller_color(&self, unk0: ::ipcdefs::nn::util::Unorm8x4, unk1: ::ipcdefs::nn::util::Unorm8x4, unk2: ::ipcdefs::nn::hid::system::UniquePadId) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::util::Unorm8x4,
			unk1: ::ipcdefs::nn::util::Unorm8x4,
			unk2: ::ipcdefs::nn::hid::system::UniquePadId,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(221)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn connect_usb_pads_async(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn disconnect_usb_pads_async(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn update_design_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_unique_pad_driver_state(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_six_axis_sensor_driver_states(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_abstracted_pad_handles(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_abstracted_pad_state(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_abstracted_pads_state(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_auto_pilot_virtual_pad_state(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unset_auto_pilot_virtual_pad_state(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unset_all_auto_pilot_virtual_pad_state(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn add_registered_device(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IHidDebugServer<T> {
	fn from(obj: T) -> IHidDebugServer<T> {
		IHidDebugServer(obj)
	}
}
