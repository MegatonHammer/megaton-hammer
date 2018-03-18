
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IApplicationProxyService(Session);

impl IApplicationProxyService {
	pub fn new() -> Result<Arc<IApplicationProxyService>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IApplicationProxyService>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"appletOE") {
			let ret = Arc::new(IApplicationProxyService(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"appletOE").map(|s| Arc::new(unsafe { IApplicationProxyService::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IApplicationProxyService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IApplicationProxyService {
	pub fn open_application_proxy(&self, unk0: u64, unk2: &KObject) -> Result<::nn::am::service::IApplicationProxy> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(unk0)
			.send_pid()
			.copy_handle(unk2)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IApplicationProxyService {
	unsafe fn from_kobject(obj: KObject) -> IApplicationProxyService {
		IApplicationProxyService(Session::from_kobject(obj))
	}
}
