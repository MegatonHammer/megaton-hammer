
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct ISystemServiceCreator(Session);

impl ISystemServiceCreator {
	pub fn new() -> Result<Arc<ISystemServiceCreator>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<ISystemServiceCreator>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"ldn:s\0\0\0") {
			let ret = Arc::new(ISystemServiceCreator(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"ldn:s\0\0\0").map(|s| Arc::new(unsafe { ISystemServiceCreator::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for ISystemServiceCreator {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ISystemServiceCreator {
	pub fn create_system_local_communication_service(&self, ) -> Result<Session> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for ISystemServiceCreator {
	unsafe fn from_kobject(obj: KObject) -> ISystemServiceCreator {
		ISystemServiceCreator(Session::from_kobject(obj))
	}
}
