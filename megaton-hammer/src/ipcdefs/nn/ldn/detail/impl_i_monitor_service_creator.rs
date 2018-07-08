
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IMonitorServiceCreator<T>(T);

impl IMonitorServiceCreator<Session> {
	pub fn raw_new() -> Result<IMonitorServiceCreator<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"ldn:m\0\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IMonitorServiceCreator<Session>>> {
		use alloc::arc::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IMonitorServiceCreator<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"ldn:m\0\0\0") {
			let ret = Arc::new(IMonitorServiceCreator(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
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
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
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
