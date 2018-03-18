
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct LocationResolverInterface(Session);

impl LocationResolverInterface {
	pub fn new() -> Result<Arc<LocationResolverInterface>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<LocationResolverInterface>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"lr\0\0\0\0\0\0") {
			let ret = Arc::new(LocationResolverInterface(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"lr\0\0\0\0\0\0").map(|s| Arc::new(unsafe { LocationResolverInterface::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for LocationResolverInterface {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl LocationResolverInterface {
}

impl FromKObject for LocationResolverInterface {
	unsafe fn from_kobject(obj: KObject) -> LocationResolverInterface {
		LocationResolverInterface(Session::from_kobject(obj))
	}
}
