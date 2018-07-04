
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IScreenShotService<T>(T);

impl IScreenShotService<Session> {
	pub fn raw_new() -> Result<IScreenShotService<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"caps:ss\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IScreenShotService<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IScreenShotService<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"caps:ss\0") {
			let ret = Arc::new(IScreenShotService(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IScreenShotService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IScreenShotService(domain)),
			Err((sess, err)) => Err((IScreenShotService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IScreenShotService<Session>> {
		Ok(IScreenShotService(self.0.duplicate()?))
	}
}

impl<T> Deref for IScreenShotService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IScreenShotService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IScreenShotService<T> {
	// fn unknown201(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown202(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown203(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown204(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IScreenShotService<T> {
	fn from(obj: T) -> IScreenShotService<T> {
		IScreenShotService(obj)
	}
}
