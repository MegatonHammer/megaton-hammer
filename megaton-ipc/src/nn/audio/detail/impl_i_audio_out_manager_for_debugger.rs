
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IAudioOutManagerForDebugger(Session);

impl IAudioOutManagerForDebugger {
	pub fn new() -> Result<Arc<IAudioOutManagerForDebugger>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IAudioOutManagerForDebugger>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		let r = sm.get_service(*b"audout:d").map(|s| Arc::new(unsafe { IAudioOutManagerForDebugger::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IAudioOutManagerForDebugger {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IAudioOutManagerForDebugger {
	pub fn request_suspend_for_debug(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn request_resume_for_debug(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IAudioOutManagerForDebugger {
	unsafe fn from_kobject(obj: KObject) -> IAudioOutManagerForDebugger {
		IAudioOutManagerForDebugger(Session::from_kobject(obj))
	}
}
