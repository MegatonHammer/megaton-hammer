
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IHidSystemServer(Session);

impl IHidSystemServer {
	pub fn new() -> Result<Arc<IHidSystemServer>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IHidSystemServer>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"hid:sys\0") {
			let ret = Arc::new(IHidSystemServer(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"hid:sys\0").map(|s| Arc::new(unsafe { IHidSystemServer::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
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
	pub fn send_keyboard_lock_key_event(&self, unk0: ::nn::hid::system::KeyboardLockKeyEvent) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(31)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn acquire_home_button_event_handle(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(101)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn activate_home_button(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(111)
			.args(unk0)
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn acquire_sleep_button_event_handle(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(121)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn activate_sleep_button(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(131)
			.args(unk0)
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn acquire_capture_button_event_handle(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(141)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn activate_capture_button(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(151)
			.args(unk0)
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn acquire_nfc_device_update_event_handle(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(210)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn get_npads_with_nfc(&self, unk1: &mut [u32]) -> Result<i64> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(211)
			.args(())
			.descriptor(IPCBuffer::from_mut_slice(unk1, 0xa))
			;
		let res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn acquire_nfc_activate_event_handle(&self, unk0: u32) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(212)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn activate_nfc(&self, unk0: bool, unk1: u32, unk2: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn acquire_ir_sensor_event_handle(&self, unk0: u32) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(230)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn activate_ir_sensor(&self, unk0: bool, unk1: u32, unk2: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn activate_npad_system(&self, unk0: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(301)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn apply_npad_system_common_policy(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(303)
			.args(unk0)
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn enable_assigning_single_on_sl_sr_press(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(304)
			.args(unk0)
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn disable_assigning_single_on_sl_sr_press(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(305)
			.args(unk0)
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_last_active_npad(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(306)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_npad_system_ext_style(&self, unk0: u32) -> Result<(i64, i64)> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(307)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: i64,
			unk2: i64,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().unk2.clone()))
	}

	pub fn set_npad_player_led_blinking_device(&self, unk0: u32, unk1: ::nn::hid::system::DeviceType, unk2: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_unique_pads_from_npad(&self, unk0: u32, unk2: &mut [::nn::hid::system::UniquePadId]) -> Result<i64> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(321)
			.args(unk0)
			.descriptor(IPCBuffer::from_mut_slice(unk2, 0xa))
			;
		let res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_ir_sensor_state(&self, unk0: u32, unk1: ::nn::applet::AppletResourceUserId) -> Result<i64> {
		use megaton_hammer::ipc::{Request, Response};

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
		let res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_xcd_handle_for_npad_with_ir_sensor(&self, unk0: u32, unk1: ::nn::applet::AppletResourceUserId) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

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
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_applet_resource_user_id(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(500)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn register_applet_resource_user_id(&self, unk0: bool, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unregister_applet_resource_user_id(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(502)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn enable_applet_to_get_input(&self, unk0: bool, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_aruid_valid_for_vibration(&self, unk0: bool, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn enable_applet_to_get_six_axis_sensor(&self, unk0: bool, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_vibration_master_volume(&self, unk0: f32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(510)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_vibration_master_volume(&self, ) -> Result<f32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(511)
			.args(())
			;
		let res : Response<f32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn begin_permit_vibration_session(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(512)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn end_permit_vibration_session(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(513)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn enable_handheld_hids(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(520)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn disable_handheld_hids(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(521)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn acquire_play_report_controller_usage_update_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(540)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn get_play_report_controller_usages(&self, unk1: &mut [::nn::hid::system::PlayReportControllerUsage]) -> Result<i64> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(541)
			.args(())
			.descriptor(IPCBuffer::from_mut_slice(unk1, 0xa))
			;
		let res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn acquire_play_report_registered_device_update_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(542)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn get_registered_devices(&self, unk1: &mut [::nn::hid::system::RegisteredDevice]) -> Result<i64> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(543)
			.args(())
			.descriptor(IPCBuffer::from_mut_slice(unk1, 0xa))
			;
		let res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn acquire_connection_trigger_timeout_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(544)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn send_connection_trigger(&self, unk0: ::nn::bluetooth::Address) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(545)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn acquire_device_registered_event_for_controller_support(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(546)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn get_allowed_bluetooth_links_count(&self, ) -> Result<i64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(547)
			.args(())
			;
		let res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn activate_unique_pad(&self, unk0: ::nn::applet::AppletResourceUserId, unk1: ::nn::hid::system::UniquePadId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn acquire_unique_pad_connection_event_handle(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(702)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn get_unique_pad_ids(&self, unk1: &mut [::nn::hid::system::UniquePadId]) -> Result<i64> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(703)
			.args(())
			.descriptor(IPCBuffer::from_mut_slice(unk1, 0xa))
			;
		let res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn acquire_joy_detach_on_bluetooth_off_event_handle(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(751)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn list_six_axis_sensor_handles(&self, unk0: ::nn::hid::system::UniquePadId, unk2: &mut [::nn::hid::system::UniqueSixAxisSensorHandle]) -> Result<i64> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(800)
			.args(unk0)
			.descriptor(IPCBuffer::from_mut_slice(unk2, 0xa))
			;
		let res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn is_six_axis_sensor_user_calibration_supported(&self, unk0: ::nn::hid::system::UniqueSixAxisSensorHandle) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(801)
			.args(unk0)
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn reset_six_axis_sensor_calibration_values(&self, unk0: ::nn::hid::system::UniqueSixAxisSensorHandle) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(802)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn start_six_axis_sensor_user_calibration(&self, unk0: ::nn::hid::system::UniqueSixAxisSensorHandle) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(803)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn cancel_six_axis_sensor_user_calibration(&self, unk0: ::nn::hid::system::UniqueSixAxisSensorHandle) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(804)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_unique_pad_bluetooth_address(&self, unk0: ::nn::hid::system::UniquePadId) -> Result<::nn::bluetooth::Address> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(805)
			.args(unk0)
			;
		let res : Response<::nn::bluetooth::Address> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn disconnect_unique_pad(&self, unk0: ::nn::hid::system::UniquePadId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(806)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn start_analog_stick_manual_calibration(&self, unk0: ::nn::hid::system::UniquePadId, unk1: i64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn retry_current_analog_stick_manual_calibration_stage(&self, unk0: ::nn::hid::system::UniquePadId, unk1: i64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn cancel_analog_stick_manual_calibration(&self, unk0: ::nn::hid::system::UniquePadId, unk1: i64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn reset_analog_stick_manual_calibration(&self, unk0: ::nn::hid::system::UniquePadId, unk1: i64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_usb_full_key_controller_enabled(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(850)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn enable_usb_full_key_controller(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(851)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_usb_connected(&self, unk0: ::nn::hid::system::UniquePadId) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(852)
			.args(unk0)
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn activate_input_detector(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(900)
			.args(unk0)
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn notify_input_detector(&self, unk0: ::nn::hid::system::InputSourceId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(901)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn initialize_firmware_update(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1000)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_firmware_version(&self, unk0: ::nn::hid::system::UniquePadId) -> Result<::nn::hid::system::FirmwareVersion> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1001)
			.args(unk0)
			;
		let res : Response<::nn::hid::system::FirmwareVersion> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_available_firmware_version(&self, unk0: ::nn::hid::system::UniquePadId) -> Result<::nn::hid::system::FirmwareVersion> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1002)
			.args(unk0)
			;
		let res : Response<::nn::hid::system::FirmwareVersion> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn is_firmware_update_available(&self, unk0: ::nn::hid::system::UniquePadId) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1003)
			.args(unk0)
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn check_firmware_update_required(&self, unk0: ::nn::hid::system::UniquePadId) -> Result<i64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1004)
			.args(unk0)
			;
		let res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn start_firmware_update(&self, unk0: ::nn::hid::system::UniquePadId) -> Result<::nn::hid::system::FirmwareUpdateDeviceHandle> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1005)
			.args(unk0)
			;
		let res : Response<::nn::hid::system::FirmwareUpdateDeviceHandle> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn abort_firmware_update(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1006)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_firmware_update_state(&self, unk0: ::nn::hid::system::FirmwareUpdateDeviceHandle) -> Result<::nn::hid::system::FirmwareUpdateState> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1007)
			.args(unk0)
			;
		let res : Response<::nn::hid::system::FirmwareUpdateState> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IHidSystemServer {
	unsafe fn from_kobject(obj: KObject) -> IHidSystemServer {
		IHidSystemServer(Session::from_kobject(obj))
	}
}
