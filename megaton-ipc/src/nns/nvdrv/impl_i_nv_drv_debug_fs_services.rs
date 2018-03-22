
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct INvDrvDebugFSServices(Session);

impl INvDrvDebugFSServices {
	pub fn new() -> Result<Arc<INvDrvDebugFSServices>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<INvDrvDebugFSServices>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"nvdrvdbg") {
			let ret = Arc::new(INvDrvDebugFSServices(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"nvdrvdbg").map(|s| Arc::new(unsafe { INvDrvDebugFSServices::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for INvDrvDebugFSServices {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl INvDrvDebugFSServices {
	// fn open_log(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn close_log(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn read_log(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for INvDrvDebugFSServices {
	unsafe fn from_kobject(obj: KObject) -> INvDrvDebugFSServices {
		INvDrvDebugFSServices(Session::from_kobject(obj))
	}
}
