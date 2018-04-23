
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IMonitorServiceCreator<T>(T);

impl IMonitorServiceCreator<Session> {
	pub fn new() -> Result<Arc<IMonitorServiceCreator<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IMonitorServiceCreator<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"ldn:m\0\0\0") {
			let ret = Arc::new(IMonitorServiceCreator(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"ldn:m\0\0\0").map(|s: KObject| Arc::new(Session::from(s).into()));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}

	pub fn to_domain(self) -> ::core::result::Result<IMonitorServiceCreator<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IMonitorServiceCreator(domain)),
			Err((sess, err)) => Err((IMonitorServiceCreator(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IMonitorServiceCreator<Session>> {
		Ok(IMonitorServiceCreator(self.0.duplicate()?))
	}
}

impl<T> Deref for IMonitorServiceCreator<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IMonitorServiceCreator<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IMonitorServiceCreator<T> {
	pub fn create_monitor_service(&self, ) -> Result<T> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

}

impl<T: Object> From<T> for IMonitorServiceCreator<T> {
	fn from(obj: T) -> IMonitorServiceCreator<T> {
		IMonitorServiceCreator(obj)
	}
}
