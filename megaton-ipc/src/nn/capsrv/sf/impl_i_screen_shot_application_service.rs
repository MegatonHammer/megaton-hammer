
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IScreenShotApplicationService(Session);

impl IScreenShotApplicationService {
	pub fn new() -> Result<Arc<IScreenShotApplicationService>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IScreenShotApplicationService>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"caps:su\0") {
			let ret = Arc::new(IScreenShotApplicationService(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"caps:su\0").map(|s| Arc::new(unsafe { IScreenShotApplicationService::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IScreenShotApplicationService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IScreenShotApplicationService {
	// fn save_screen_shot(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn save_screen_shot_ex0(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IScreenShotApplicationService {
	unsafe fn from_kobject(obj: KObject) -> IScreenShotApplicationService {
		IScreenShotApplicationService(Session::from_kobject(obj))
	}
}
