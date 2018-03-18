
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IPrivateService(Session);

impl IPrivateService {
	pub fn new() -> Result<Arc<IPrivateService>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IPrivateService>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		let r = sm.get_service(*b"fatal:p\0").map(|s| Arc::new(unsafe { IPrivateService::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IPrivateService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IPrivateService {
	pub fn unknown0(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

}

impl FromKObject for IPrivateService {
	unsafe fn from_kobject(obj: KObject) -> IPrivateService {
		IPrivateService(Session::from_kobject(obj))
	}
}
