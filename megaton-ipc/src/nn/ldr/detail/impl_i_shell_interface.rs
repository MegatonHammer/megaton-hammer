
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IShellInterface(Session);

impl IShellInterface {
	pub fn new() -> Result<Arc<IShellInterface>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IShellInterface>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"ldr:shel") {
			let ret = Arc::new(IShellInterface(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"ldr:shel").map(|s| Arc::new(unsafe { IShellInterface::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IShellInterface {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IShellInterface {
	// fn add_process_to_launch_queue(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn clear_launch_queue(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IShellInterface {
	unsafe fn from_kobject(obj: KObject) -> IShellInterface {
		IShellInterface(Session::from_kobject(obj))
	}
}
