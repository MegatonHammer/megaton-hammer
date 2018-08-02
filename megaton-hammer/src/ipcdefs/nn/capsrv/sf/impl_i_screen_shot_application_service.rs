
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::sync::Arc;

#[derive(Debug)]
pub struct IScreenShotApplicationService<T>(T);

impl IScreenShotApplicationService<Session> {
	pub fn raw_new() -> Result<IScreenShotApplicationService<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"caps:su\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IScreenShotApplicationService<Session>>> {
		use alloc::sync::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IScreenShotApplicationService<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"caps:su\0") {
			let ret = Arc::new(IScreenShotApplicationService(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IScreenShotApplicationService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IScreenShotApplicationService(domain)),
			Err((sess, err)) => Err((IScreenShotApplicationService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IScreenShotApplicationService<Session>> {
		Ok(IScreenShotApplicationService(self.0.duplicate()?))
	}
}

impl<T> Deref for IScreenShotApplicationService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IScreenShotApplicationService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IScreenShotApplicationService<T> {
	// fn save_screen_shot(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn save_screen_shot_ex0(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IScreenShotApplicationService<T> {
	fn from(obj: T) -> IScreenShotApplicationService<T> {
		IScreenShotApplicationService(obj)
	}
}
