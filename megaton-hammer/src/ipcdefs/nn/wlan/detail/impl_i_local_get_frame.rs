
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct ILocalGetFrame<T>(T);

impl ILocalGetFrame<Session> {
	pub fn raw_new() -> Result<ILocalGetFrame<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"wlan:lg\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<ILocalGetFrame<Session>>> {
		use alloc::arc::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<ILocalGetFrame<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"wlan:lg\0") {
			let ret = Arc::new(ILocalGetFrame(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<ILocalGetFrame<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ILocalGetFrame(domain)),
			Err((sess, err)) => Err((ILocalGetFrame(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ILocalGetFrame<Session>> {
		Ok(ILocalGetFrame(self.0.duplicate()?))
	}
}

impl<T> Deref for ILocalGetFrame<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ILocalGetFrame<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ILocalGetFrame<T> {
	// fn unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for ILocalGetFrame<T> {
	fn from(obj: T) -> ILocalGetFrame<T> {
		ILocalGetFrame(obj)
	}
}
