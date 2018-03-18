
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IHidTemporaryServer(Session);

impl IHidTemporaryServer {
	pub fn new() -> Result<Arc<IHidTemporaryServer>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IHidTemporaryServer>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"hid:tmp\0") {
			let ret = Arc::new(IHidTemporaryServer(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"hid:tmp\0").map(|s| Arc::new(unsafe { IHidTemporaryServer::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IHidTemporaryServer {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IHidTemporaryServer {
	pub fn get_console_six_axis_sensor_calibration_values(&self, unk0: ::nn::hid::ConsoleSixAxisSensorHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<::nn::hid::tmp::ConsoleSixAxisSensorCalibrationValues> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::hid::ConsoleSixAxisSensorHandle,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(0)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let res : Response<::nn::hid::tmp::ConsoleSixAxisSensorCalibrationValues> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IHidTemporaryServer {
	unsafe fn from_kobject(obj: KObject) -> IHidTemporaryServer {
		IHidTemporaryServer(Session::from_kobject(obj))
	}
}
