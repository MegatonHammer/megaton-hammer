
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IStaticService(Session);

impl IStaticService {
	pub fn new() -> Result<Arc<IStaticService>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IStaticService>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		let r = sm.get_service(*b"mii:e\0\0\0").map(|s| Arc::new(unsafe { IStaticService::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}

		let r = sm.get_service(*b"mii:u\0\0\0").map(|s| Arc::new(unsafe { IStaticService::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IStaticService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IStaticService {
	pub fn get_database_service_shared_pointer(&self, unk0: i32) -> Result<::nn::mii::detail::IDatabaseService> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IStaticService {
	unsafe fn from_kobject(obj: KObject) -> IStaticService {
		IStaticService(Session::from_kobject(obj))
	}
}
