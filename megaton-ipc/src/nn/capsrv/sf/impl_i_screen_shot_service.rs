
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IScreenShotService(Session);

impl IScreenShotService {
	pub fn new() -> Result<Arc<IScreenShotService>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IScreenShotService>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"caps:ss\0") {
			let ret = Arc::new(IScreenShotService(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"caps:ss\0").map(|s| Arc::new(unsafe { IScreenShotService::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IScreenShotService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IScreenShotService {
	// fn unknown201(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown202(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown203(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown204(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IScreenShotService {
	unsafe fn from_kobject(obj: KObject) -> IScreenShotService {
		IScreenShotService(Session::from_kobject(obj))
	}
}
