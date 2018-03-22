
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct INvDrvServices(Session);

impl INvDrvServices {
	pub fn new_nvdrv_s() -> Result<Arc<INvDrvServices>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<INvDrvServices>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"nvdrv:s\0") {
			let ret = Arc::new(INvDrvServices(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"nvdrv:s\0").map(|s| Arc::new(unsafe { INvDrvServices::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
	pub fn new_nvdrv_t() -> Result<Arc<INvDrvServices>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<INvDrvServices>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"nvdrv:t\0") {
			let ret = Arc::new(INvDrvServices(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"nvdrv:t\0").map(|s| Arc::new(unsafe { INvDrvServices::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
	pub fn new_nvdrv_a() -> Result<Arc<INvDrvServices>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<INvDrvServices>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"nvdrv:a\0") {
			let ret = Arc::new(INvDrvServices(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"nvdrv:a\0").map(|s| Arc::new(unsafe { INvDrvServices::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
	pub fn new_nvdrv() -> Result<Arc<INvDrvServices>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<INvDrvServices>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"nvdrv\0\0\0") {
			let ret = Arc::new(INvDrvServices(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"nvdrv\0\0\0").map(|s| Arc::new(unsafe { INvDrvServices::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for INvDrvServices {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl INvDrvServices {
	// fn open(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn ioctl(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn close(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn initialize(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn query_event(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn map_shared_mem(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_status(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn force_set_client_pid(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_client_pid(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn dump_graphics_memory_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown10(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn ioctl2(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn ioctl3(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown13(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for INvDrvServices {
	unsafe fn from_kobject(obj: KObject) -> INvDrvServices {
		INvDrvServices(Session::from_kobject(obj))
	}
}
