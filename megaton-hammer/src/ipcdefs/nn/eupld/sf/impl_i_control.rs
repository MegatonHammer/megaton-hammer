
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IControl<T>(T);

impl IControl<Session> {
	pub fn raw_new() -> Result<IControl<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"eupld:c\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IControl<Session>>> {
		use alloc::arc::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IControl<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"eupld:c\0") {
			let ret = Arc::new(IControl(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IControl<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IControl(domain)),
			Err((sess, err)) => Err((IControl(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IControl<Session>> {
		Ok(IControl(self.0.duplicate()?))
	}
}

impl<T> Deref for IControl<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IControl<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IControl<T> {
	// fn unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown3(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IControl<T> {
	fn from(obj: T) -> IControl<T> {
		IControl(obj)
	}
}
