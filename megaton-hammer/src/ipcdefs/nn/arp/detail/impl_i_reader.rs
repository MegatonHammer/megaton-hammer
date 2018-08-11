
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::sync::Arc;

#[derive(Debug)]
pub struct IReader<T>(T);

impl IReader<Session> {
	pub fn raw_new() -> Result<IReader<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"arp:r\0\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IReader<Session>>> {
		use alloc::sync::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IReader<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"arp:r\0\0\0") {
			let ret = Arc::new(IReader(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IReader<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IReader(domain)),
			Err((sess, err)) => Err((IReader(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IReader<Session>> {
		Ok(IReader(self.0.duplicate()?))
	}
}

impl<T> Deref for IReader<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IReader<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IReader<T> {
	// fn get_application_launch_property(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_application_launch_property_with_application_id(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_application_control_property(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_application_control_property_with_application_id(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IReader<T> {
	fn from(obj: T) -> IReader<T> {
		IReader(obj)
	}
}
