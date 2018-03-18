
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IArbitrationManager(Session);

impl IArbitrationManager {
	pub fn new() -> Result<Arc<IArbitrationManager>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IArbitrationManager>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"pcv:arb\0") {
			let ret = Arc::new(IArbitrationManager(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"pcv:arb\0").map(|s| Arc::new(unsafe { IArbitrationManager::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IArbitrationManager {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IArbitrationManager {
	pub fn release_control(&self, unk0: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IArbitrationManager {
	unsafe fn from_kobject(obj: KObject) -> IArbitrationManager {
		IArbitrationManager(Session::from_kobject(obj))
	}
}
