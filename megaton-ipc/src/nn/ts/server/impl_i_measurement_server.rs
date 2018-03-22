
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IMeasurementServer(Session);

impl IMeasurementServer {
	pub fn new() -> Result<Arc<IMeasurementServer>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IMeasurementServer>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"ts\0\0\0\0\0\0") {
			let ret = Arc::new(IMeasurementServer(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"ts\0\0\0\0\0\0").map(|s| Arc::new(unsafe { IMeasurementServer::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IMeasurementServer {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IMeasurementServer {
	// fn unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown3(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IMeasurementServer {
	unsafe fn from_kobject(obj: KObject) -> IMeasurementServer {
		IMeasurementServer(Session::from_kobject(obj))
	}
}
