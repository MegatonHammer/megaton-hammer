
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct INvGemControl<T>(T);

impl INvGemControl<Session> {
	pub fn raw_new() -> Result<INvGemControl<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"nvgem:c\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<INvGemControl<Session>>> {
		use alloc::arc::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<INvGemControl<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"nvgem:c\0") {
			let ret = Arc::new(INvGemControl(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<INvGemControl<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(INvGemControl(domain)),
			Err((sess, err)) => Err((INvGemControl(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<INvGemControl<Session>> {
		Ok(INvGemControl(self.0.duplicate()?))
	}
}

impl<T> Deref for INvGemControl<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for INvGemControl<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> INvGemControl<T> {
	// fn unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown3(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown4(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown5(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown6(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown7(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for INvGemControl<T> {
	fn from(obj: T) -> INvGemControl<T> {
		INvGemControl(obj)
	}
}
