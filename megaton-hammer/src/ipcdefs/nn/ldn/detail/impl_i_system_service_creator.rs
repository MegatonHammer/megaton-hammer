
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct ISystemServiceCreator<T>(T);

impl ISystemServiceCreator<Session> {
	pub fn raw_new() -> Result<ISystemServiceCreator<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"ldn:s\0\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<ISystemServiceCreator<Session>>> {
		use alloc::arc::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<ISystemServiceCreator<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"ldn:s\0\0\0") {
			let ret = Arc::new(ISystemServiceCreator(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<ISystemServiceCreator<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ISystemServiceCreator(domain)),
			Err((sess, err)) => Err((ISystemServiceCreator(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ISystemServiceCreator<Session>> {
		Ok(ISystemServiceCreator(self.0.duplicate()?))
	}
}

impl<T> Deref for ISystemServiceCreator<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ISystemServiceCreator<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ISystemServiceCreator<T> {
	pub fn create_system_local_communication_service(&self, ) -> Result<T> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

}

impl<T: Object> From<T> for ISystemServiceCreator<T> {
	fn from(obj: T) -> ISystemServiceCreator<T> {
		ISystemServiceCreator(obj)
	}
}
