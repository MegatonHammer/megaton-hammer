
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IParentalControlServiceFactory(Session);

impl IParentalControlServiceFactory {
	pub fn new() -> Result<Arc<IParentalControlServiceFactory>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IParentalControlServiceFactory>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"pctl:s\0\0") {
			let ret = Arc::new(IParentalControlServiceFactory(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"pctl:s\0\0").map(|s| Arc::new(unsafe { IParentalControlServiceFactory::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"pctl:r\0\0") {
			let ret = Arc::new(IParentalControlServiceFactory(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"pctl:r\0\0").map(|s| Arc::new(unsafe { IParentalControlServiceFactory::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"pctl:a\0\0") {
			let ret = Arc::new(IParentalControlServiceFactory(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"pctl:a\0\0").map(|s| Arc::new(unsafe { IParentalControlServiceFactory::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"pctl\0\0\0\0") {
			let ret = Arc::new(IParentalControlServiceFactory(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"pctl\0\0\0\0").map(|s| Arc::new(unsafe { IParentalControlServiceFactory::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IParentalControlServiceFactory {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IParentalControlServiceFactory {
	pub fn get_service(&self, unk0: u64) -> Result<::nn::pctl::detail::ipc::IParentalControlService> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IParentalControlServiceFactory {
	unsafe fn from_kobject(obj: KObject) -> IParentalControlServiceFactory {
		IParentalControlServiceFactory(Session::from_kobject(obj))
	}
}
