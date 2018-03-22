
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IReader(Session);

impl IReader {
	pub fn new() -> Result<Arc<IReader>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IReader>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"arp:r\0\0\0") {
			let ret = Arc::new(IReader(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"arp:r\0\0\0").map(|s| Arc::new(unsafe { IReader::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IReader {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IReader {
	// fn read_header0(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn read_header1(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn read_data0(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn read_data1(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IReader {
	unsafe fn from_kobject(obj: KObject) -> IReader {
		IReader(Session::from_kobject(obj))
	}
}
