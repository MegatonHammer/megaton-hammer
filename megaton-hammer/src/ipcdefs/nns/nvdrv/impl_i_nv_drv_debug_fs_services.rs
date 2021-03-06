
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::sync::Arc;

#[derive(Debug)]
pub struct INvDrvDebugFSServices<T>(T);

impl INvDrvDebugFSServices<Session> {
	pub fn raw_new() -> Result<INvDrvDebugFSServices<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"nvdrvdbg")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<INvDrvDebugFSServices<Session>>> {
		use alloc::sync::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<INvDrvDebugFSServices<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"nvdrvdbg") {
			let ret = Arc::new(INvDrvDebugFSServices(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<INvDrvDebugFSServices<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(INvDrvDebugFSServices(domain)),
			Err((sess, err)) => Err((INvDrvDebugFSServices(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<INvDrvDebugFSServices<Session>> {
		Ok(INvDrvDebugFSServices(self.0.duplicate()?))
	}
}

impl<T> Deref for INvDrvDebugFSServices<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for INvDrvDebugFSServices<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> INvDrvDebugFSServices<T> {
	// fn open_log(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn close_log(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn read_log(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown3(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown4(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for INvDrvDebugFSServices<T> {
	fn from(obj: T) -> INvDrvDebugFSServices<T> {
		INvDrvDebugFSServices(obj)
	}
}
