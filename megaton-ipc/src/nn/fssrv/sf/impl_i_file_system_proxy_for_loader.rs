
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IFileSystemProxyForLoader(Session);

impl IFileSystemProxyForLoader {
	pub fn new() -> Result<Arc<IFileSystemProxyForLoader>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IFileSystemProxyForLoader>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		let r = sm.get_service(*b"fsp-ldr\0").map(|s| Arc::new(unsafe { IFileSystemProxyForLoader::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IFileSystemProxyForLoader {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IFileSystemProxyForLoader {
	// fn mount_code(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn is_code_mounted(&self, tid: ::nn::ApplicationId) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(tid)
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IFileSystemProxyForLoader {
	unsafe fn from_kobject(obj: KObject) -> IFileSystemProxyForLoader {
		IFileSystemProxyForLoader(Session::from_kobject(obj))
	}
}
