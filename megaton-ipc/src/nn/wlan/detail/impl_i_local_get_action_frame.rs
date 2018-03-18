
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct ILocalGetActionFrame(Session);

impl ILocalGetActionFrame {
	pub fn new() -> Result<Arc<ILocalGetActionFrame>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<ILocalGetActionFrame>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		let r = sm.get_service(*b"wlan:lga").map(|s| Arc::new(unsafe { ILocalGetActionFrame::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for ILocalGetActionFrame {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ILocalGetActionFrame {
	pub fn unknown0(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for ILocalGetActionFrame {
	unsafe fn from_kobject(obj: KObject) -> ILocalGetActionFrame {
		ILocalGetActionFrame(Session::from_kobject(obj))
	}
}
