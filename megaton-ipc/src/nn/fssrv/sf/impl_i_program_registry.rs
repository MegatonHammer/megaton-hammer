
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IProgramRegistry(Session);

impl IProgramRegistry {
	pub fn new() -> Result<Arc<IProgramRegistry>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IProgramRegistry>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		let r = sm.get_service(*b"fsp-pr\0\0").map(|s| Arc::new(unsafe { IProgramRegistry::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IProgramRegistry {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IProgramRegistry {
	// fn set_fs_permissions(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn clear_fs_permissions(&self, pid: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(pid)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_enabled_program_verification(&self, enabled: u8) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(256)
			.args(enabled)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IProgramRegistry {
	unsafe fn from_kobject(obj: KObject) -> IProgramRegistry {
		IProgramRegistry(Session::from_kobject(obj))
	}
}
