
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct ISocketGetFrame(Session);

impl ISocketGetFrame {
	pub fn new() -> Result<Arc<ISocketGetFrame>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<ISocketGetFrame>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"wlan:sg\0") {
			let ret = Arc::new(ISocketGetFrame(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"wlan:sg\0").map(|s| Arc::new(unsafe { ISocketGetFrame::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for ISocketGetFrame {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ISocketGetFrame {
	// fn unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for ISocketGetFrame {
	unsafe fn from_kobject(obj: KObject) -> ISocketGetFrame {
		ISocketGetFrame(Session::from_kobject(obj))
	}
}
