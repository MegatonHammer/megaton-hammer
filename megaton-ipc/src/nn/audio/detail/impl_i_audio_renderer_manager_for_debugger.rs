
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IAudioRendererManagerForDebugger(Session);

impl IAudioRendererManagerForDebugger {
	pub fn new() -> Result<Arc<IAudioRendererManagerForDebugger>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IAudioRendererManagerForDebugger>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		let r = sm.get_service(*b"audren:d").map(|s| Arc::new(unsafe { IAudioRendererManagerForDebugger::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IAudioRendererManagerForDebugger {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IAudioRendererManagerForDebugger {
	pub fn unknown0(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown1(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IAudioRendererManagerForDebugger {
	unsafe fn from_kobject(obj: KObject) -> IAudioRendererManagerForDebugger {
		IAudioRendererManagerForDebugger(Session::from_kobject(obj))
	}
}
