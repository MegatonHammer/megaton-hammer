
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct ILocalGetActionFrame<T>(T);

impl ILocalGetActionFrame<Session> {
	pub fn raw_new() -> Result<ILocalGetActionFrame<Session>> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"wlan:lga")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<ILocalGetActionFrame<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<ILocalGetActionFrame<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"wlan:lga") {
			let ret = Arc::new(ILocalGetActionFrame(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<ILocalGetActionFrame<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ILocalGetActionFrame(domain)),
			Err((sess, err)) => Err((ILocalGetActionFrame(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ILocalGetActionFrame<Session>> {
		Ok(ILocalGetActionFrame(self.0.duplicate()?))
	}
}

impl<T> Deref for ILocalGetActionFrame<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ILocalGetActionFrame<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ILocalGetActionFrame<T> {
	// fn unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for ILocalGetActionFrame<T> {
	fn from(obj: T) -> ILocalGetActionFrame<T> {
		ILocalGetActionFrame(obj)
	}
}
