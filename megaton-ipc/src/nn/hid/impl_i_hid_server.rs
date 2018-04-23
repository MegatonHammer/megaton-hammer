
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IHidServer<T>(T);

impl IHidServer<Session> {
	pub fn new() -> Result<Arc<IHidServer<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IHidServer<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"hid\0\0\0\0\0") {
			let ret = Arc::new(IHidServer(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"hid\0\0\0\0\0").map(|s: KObject| Arc::new(Session::from(s).into()));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}

	pub fn to_domain(self) -> ::core::result::Result<IHidServer<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IHidServer(domain)),
			Err((sess, err)) => Err((IHidServer(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IHidServer<Session>> {
		Ok(IHidServer(self.0.duplicate()?))
	}
}

impl<T> Deref for IHidServer<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IHidServer<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IHidServer<T> {
	pub fn create_applet_resource(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<::nn::hid::IAppletResource<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn activate_debug_pad(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(unk0)
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn activate_touch_screen(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(11)
			.args(unk0)
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn activate_mouse(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(21)
			.args(unk0)
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn activate_keyboard(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(31)
			.args(unk0)
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn acquire_xpad_id_event_handle(&self, unk0: u64) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(40)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn release_xpad_id_event_handle(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(41)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn activate_xpad(&self, unk0: ::nn::hid::BasicXpadId, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_xpad_ids(&self, unk1: &mut [::nn::hid::BasicXpadId]) -> Result<i64> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(55)
			.args(())
			.descriptor(IPCBuffer::from_mut_slice(unk1, 0xa))
			;
		let res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn activate_joy_xpad(&self, unk0: ::nn::hid::JoyXpadId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(56)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_joy_xpad_lifo_handle(&self, unk0: ::nn::hid::JoyXpadId) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(58)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn get_joy_xpad_ids(&self, unk1: &mut [::nn::hid::JoyXpadId]) -> Result<i64> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(59)
			.args(())
			.descriptor(IPCBuffer::from_mut_slice(unk1, 0xa))
			;
		let res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn activate_six_axis_sensor(&self, unk0: ::nn::hid::BasicXpadId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(60)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn deactivate_six_axis_sensor(&self, unk0: ::nn::hid::BasicXpadId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(61)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_six_axis_sensor_lifo_handle(&self, unk0: ::nn::hid::BasicXpadId) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(62)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn activate_joy_six_axis_sensor(&self, unk0: ::nn::hid::JoyXpadId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(63)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn deactivate_joy_six_axis_sensor(&self, unk0: ::nn::hid::JoyXpadId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(64)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_joy_six_axis_sensor_lifo_handle(&self, unk0: ::nn::hid::JoyXpadId) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(65)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn start_six_axis_sensor(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn stop_six_axis_sensor(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_six_axis_sensor_fusion_enabled(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

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
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn enable_six_axis_sensor_fusion(&self, unk0: bool, unk1: ::nn::hid::SixAxisSensorHandle, unk2: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_six_axis_sensor_fusion_parameters(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: f32, unk2: f32, unk3: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_six_axis_sensor_fusion_parameters(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<(f32, f32)> {
		use megaton_hammer::ipc::{Request, Response};

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
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk3.clone(),res.get_raw().unk4.clone()))
	}

	pub fn reset_six_axis_sensor_fusion_parameters(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_accelerometer_parameters(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: f32, unk2: f32, unk3: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_accelerometer_parameters(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<(f32, f32)> {
		use megaton_hammer::ipc::{Request, Response};

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
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk3.clone(),res.get_raw().unk4.clone()))
	}

	pub fn reset_accelerometer_parameters(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_accelerometer_play_mode(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: u32, unk2: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_accelerometer_play_mode(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

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
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn reset_accelerometer_play_mode(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_gyroscope_zero_drift_mode(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: u32, unk2: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_gyroscope_zero_drift_mode(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

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
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn reset_gyroscope_zero_drift_mode(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_six_axis_sensor_at_rest(&self, unk0: ::nn::hid::SixAxisSensorHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

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
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn activate_gesture(&self, unk0: i32, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_supported_npad_style_set(&self, unk0: ::nn::hid::NpadStyleTag, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_supported_npad_style_set(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<::nn::hid::NpadStyleTag> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(101)
			.args(unk0)
			.send_pid()
			;
		let res : Response<::nn::hid::NpadStyleTag> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_supported_npad_id_type(&self, unk0: ::nn::applet::AppletResourceUserId, unk2: &[u32]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(102)
			.args(unk0)
			.send_pid()
			.descriptor(IPCBuffer::from_slice(unk2, 9))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn activate_npad(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(103)
			.args(unk0)
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn deactivate_npad(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(104)
			.args(unk0)
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn acquire_npad_style_set_update_event_handle(&self, unk0: u32, unk1: ::nn::applet::AppletResourceUserId, unk2: u64) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

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

	pub fn disconnect_npad(&self, unk0: u32, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_player_led_pattern(&self, unk0: u32) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(108)
			.args(unk0)
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_npad_joy_hold_type(&self, unk0: ::nn::applet::AppletResourceUserId, unk1: i64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_npad_joy_hold_type(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<i64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(121)
			.args(unk0)
			.send_pid()
			;
		let res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_npad_joy_assignment_mode_single_by_default(&self, unk0: u32, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_npad_joy_assignment_mode_single(&self, unk0: u32, unk1: ::nn::applet::AppletResourceUserId, unk2: i64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_npad_joy_assignment_mode_dual(&self, unk0: u32, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn merge_single_joy_as_dual_joy(&self, unk0: u32, unk1: u32, unk2: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn start_lr_assignment_mode(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(126)
			.args(unk0)
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn stop_lr_assignment_mode(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(127)
			.args(unk0)
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_npad_handheld_activation_mode(&self, unk0: ::nn::applet::AppletResourceUserId, unk1: i64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_npad_handheld_activation_mode(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<i64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(129)
			.args(unk0)
			.send_pid()
			;
		let res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn swap_npad_assignment(&self, unk0: u32, unk1: u32, unk2: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_unintended_home_button_input_protection_enabled(&self, unk0: u32, unk1: ::nn::applet::AppletResourceUserId) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

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
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn enable_unintended_home_button_input_protection(&self, unk0: bool, unk1: u32, unk2: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_vibration_device_info(&self, unk0: ::nn::hid::VibrationDeviceHandle) -> Result<::nn::hid::VibrationDeviceInfoForIpc> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(200)
			.args(unk0)
			;
		let res : Response<::nn::hid::VibrationDeviceInfoForIpc> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn send_vibration_value(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_actual_vibration_value(&self, unk0: ::nn::hid::VibrationDeviceHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<::nn::hid::VibrationValue> {
		use megaton_hammer::ipc::{Request, Response};

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
		let res : Response<::nn::hid::VibrationValue> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn create_active_vibration_device_list(&self, ) -> Result<::nn::hid::IActiveVibrationDeviceList<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(203)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn permit_vibration(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(204)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_vibration_permitted(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(205)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn send_vibration_values(&self, unk0: ::nn::applet::AppletResourceUserId, unk1: &[::nn::hid::VibrationDeviceHandle], unk2: &[::nn::hid::VibrationValue]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(206)
			.args(unk0)
			.descriptor(IPCBuffer::from_slice(unk1, 9))
			.descriptor(IPCBuffer::from_slice(unk2, 9))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn activate_console_six_axis_sensor(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(300)
			.args(unk0)
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn start_console_six_axis_sensor(&self, unk0: ::nn::hid::ConsoleSixAxisSensorHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn stop_console_six_axis_sensor(&self, unk0: ::nn::hid::ConsoleSixAxisSensorHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_usb_full_key_controller_enabled(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(400)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn enable_usb_full_key_controller(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(401)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_usb_full_key_controller_connected(&self, unk0: u32) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(402)
			.args(unk0)
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_npad_communication_mode(&self, unk0: ::nn::applet::AppletResourceUserId, unk1: i64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_npad_communication_mode(&self, ) -> Result<i64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1001)
			.args(())
			;
		let res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl<T: Object> From<T> for IHidServer<T> {
	fn from(obj: T) -> IHidServer<T> {
		IHidServer(obj)
	}
}
