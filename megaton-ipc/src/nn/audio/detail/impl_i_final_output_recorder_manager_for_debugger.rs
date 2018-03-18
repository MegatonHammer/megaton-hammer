
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IFinalOutputRecorderManagerForDebugger(Session);

impl IFinalOutputRecorderManagerForDebugger {
	pub fn new() -> Result<Arc<IFinalOutputRecorderManagerForDebugger>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IFinalOutputRecorderManagerForDebugger>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"audrec:d") {
			let ret = Arc::new(IFinalOutputRecorderManagerForDebugger(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"audrec:d").map(|s| Arc::new(unsafe { IFinalOutputRecorderManagerForDebugger::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IFinalOutputRecorderManagerForDebugger {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IFinalOutputRecorderManagerForDebugger {
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

impl FromKObject for IFinalOutputRecorderManagerForDebugger {
	unsafe fn from_kobject(obj: KObject) -> IFinalOutputRecorderManagerForDebugger {
		IFinalOutputRecorderManagerForDebugger(Session::from_kobject(obj))
	}
}
