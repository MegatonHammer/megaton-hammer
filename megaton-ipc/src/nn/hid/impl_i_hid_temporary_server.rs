
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IHidTemporaryServer<T>(T);

impl IHidTemporaryServer<Session> {
	pub fn new() -> Result<Arc<IHidTemporaryServer<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IHidTemporaryServer<Session>>> = Mutex::new(Weak::new());
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

		let r = sm.get_service(*b"hid:tmp\0").map(|s: KObject| Arc::new(Session::from(s).into()));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}

	pub fn to_domain(self) -> ::core::result::Result<IHidTemporaryServer<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IHidTemporaryServer(domain)),
			Err((sess, err)) => Err((IHidTemporaryServer(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IHidTemporaryServer<Session>> {
		Ok(IHidTemporaryServer(self.0.duplicate()?))
	}
}

impl<T> Deref for IHidTemporaryServer<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IHidTemporaryServer<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IHidTemporaryServer<T> {
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

impl<T: Object> From<T> for IHidTemporaryServer<T> {
	fn from(obj: T) -> IHidTemporaryServer<T> {
		IHidTemporaryServer(obj)
	}
}
