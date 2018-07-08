
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IHidTemporaryServer<T>(T);

impl IHidTemporaryServer<Session> {
	pub fn raw_new() -> Result<IHidTemporaryServer<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"hid:tmp\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IHidTemporaryServer<Session>>> {
		use alloc::arc::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IHidTemporaryServer<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"hid:tmp\0") {
			let ret = Arc::new(IHidTemporaryServer(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
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
	pub fn get_console_six_axis_sensor_calibration_values(&self, unk0: ::ipcdefs::nn::hid::ConsoleSixAxisSensorHandle, unk1: ::ipcdefs::nn::applet::AppletResourceUserId) -> Result<::ipcdefs::nn::hid::tmp::ConsoleSixAxisSensorCalibrationValues> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::hid::ConsoleSixAxisSensorHandle,
			unk1: ::ipcdefs::nn::applet::AppletResourceUserId,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let res : Response<::ipcdefs::nn::hid::tmp::ConsoleSixAxisSensorCalibrationValues> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl<T: Object> From<T> for IHidTemporaryServer<T> {
	fn from(obj: T) -> IHidTemporaryServer<T> {
		IHidTemporaryServer(obj)
	}
}
