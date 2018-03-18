
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IAudioOutManager(Session);

impl IAudioOutManager {
	pub fn new() -> Result<Arc<IAudioOutManager>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IAudioOutManager>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		let r = sm.get_service(*b"audout:u").map(|s| Arc::new(unsafe { IAudioOutManager::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IAudioOutManager {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IAudioOutManager {
	// fn list_audio_outs(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn open_audio_out(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn list_audio_outs_ex(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn open_audio_out_ex(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IAudioOutManager {
	unsafe fn from_kobject(obj: KObject) -> IAudioOutManager {
		IAudioOutManager(Session::from_kobject(obj))
	}
}
