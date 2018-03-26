
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct LocationResolverInterface<T>(T);

impl LocationResolverInterface<Session> {
	pub fn new() -> Result<Arc<LocationResolverInterface<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<LocationResolverInterface<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"lr\0\0\0\0\0\0") {
			let ret = Arc::new(LocationResolverInterface(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"lr\0\0\0\0\0\0").map(|s: KObject| Arc::new(Session::from(s).into()));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}

	pub fn to_domain(self) -> ::core::result::Result<LocationResolverInterface<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(LocationResolverInterface(domain)),
			Err((sess, err)) => Err((LocationResolverInterface(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<LocationResolverInterface<Session>> {
		Ok(LocationResolverInterface(self.0.duplicate()?))
	}
}

impl<T> Deref for LocationResolverInterface<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for LocationResolverInterface<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> LocationResolverInterface<T> {
}

impl<T: Object> From<T> for LocationResolverInterface<T> {
	fn from(obj: T) -> LocationResolverInterface<T> {
		LocationResolverInterface(obj)
	}
}
