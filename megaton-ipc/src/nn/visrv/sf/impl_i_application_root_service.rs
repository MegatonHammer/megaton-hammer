
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IApplicationRootService(Session);

impl IApplicationRootService {
	pub fn new() -> Result<Arc<IApplicationRootService>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IApplicationRootService>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"vi:u\0\0\0\0") {
			let ret = Arc::new(IApplicationRootService(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"vi:u\0\0\0\0").map(|s| Arc::new(unsafe { IApplicationRootService::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IApplicationRootService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IApplicationRootService {
	pub fn get_display_service(&self, unk0: u32) -> Result<::nn::visrv::sf::IApplicationDisplayService> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IApplicationRootService {
	unsafe fn from_kobject(obj: KObject) -> IApplicationRootService {
		IApplicationRootService(Session::from_kobject(obj))
	}
}
