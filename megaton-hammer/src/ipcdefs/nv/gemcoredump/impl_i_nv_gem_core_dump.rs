
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct INvGemCoreDump<T>(T);

impl INvGemCoreDump<Session> {
	pub fn raw_new() -> Result<INvGemCoreDump<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"nvgem:cd")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<INvGemCoreDump<Session>>> {
		use alloc::arc::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<INvGemCoreDump<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"nvgem:cd") {
			let ret = Arc::new(INvGemCoreDump(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<INvGemCoreDump<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(INvGemCoreDump(domain)),
			Err((sess, err)) => Err((INvGemCoreDump(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<INvGemCoreDump<Session>> {
		Ok(INvGemCoreDump(self.0.duplicate()?))
	}
}

impl<T> Deref for INvGemCoreDump<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for INvGemCoreDump<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> INvGemCoreDump<T> {
	// fn unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for INvGemCoreDump<T> {
	fn from(obj: T) -> INvGemCoreDump<T> {
		INvGemCoreDump(obj)
	}
}
