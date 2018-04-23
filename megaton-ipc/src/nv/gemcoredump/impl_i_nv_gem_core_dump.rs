
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct INvGemCoreDump<T>(T);

impl INvGemCoreDump<Session> {
	pub fn new() -> Result<Arc<INvGemCoreDump<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<INvGemCoreDump<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"nvgem:cd") {
			let ret = Arc::new(INvGemCoreDump(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"nvgem:cd").map(|s: KObject| Arc::new(Session::from(s).into()));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
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
