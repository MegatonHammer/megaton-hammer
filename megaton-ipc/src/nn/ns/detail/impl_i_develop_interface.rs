
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IDevelopInterface(Session);

impl IDevelopInterface {
	pub fn new() -> Result<Arc<IDevelopInterface>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IDevelopInterface>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"ns:dev\0\0") {
			let ret = Arc::new(IDevelopInterface(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"ns:dev\0\0").map(|s| Arc::new(unsafe { IDevelopInterface::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IDevelopInterface {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IDevelopInterface {
	// fn launch_title(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn terminate_title_by_pid(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn terminate_title_by_title_id(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_ns_dev_wait_event(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_ns_dev_event_type(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn terminate_crashing_title(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn install_title(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(6)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn set_event_state6(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_event_state(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IDevelopInterface {
	unsafe fn from_kobject(obj: KObject) -> IDevelopInterface {
		IDevelopInterface(Session::from_kobject(obj))
	}
}
