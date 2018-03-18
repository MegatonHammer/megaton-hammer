
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IRandomInterface(Session);

impl IRandomInterface {
	pub fn new() -> Result<Arc<IRandomInterface>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IRandomInterface>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		let r = sm.get_service(*b"csrng\0\0\0").map(|s| Arc::new(unsafe { IRandomInterface::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IRandomInterface {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IRandomInterface {
	// fn get_random_bytes(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IRandomInterface {
	unsafe fn from_kobject(obj: KObject) -> IRandomInterface {
		IRandomInterface(Session::from_kobject(obj))
	}
}
