
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IBtmDebug<T>(T);

impl IBtmDebug<Session> {
	pub fn raw_new() -> Result<IBtmDebug<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"btm:dbg\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IBtmDebug<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IBtmDebug<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"btm:dbg\0") {
			let ret = Arc::new(IBtmDebug(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IBtmDebug<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IBtmDebug(domain)),
			Err((sess, err)) => Err((IBtmDebug(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IBtmDebug<Session>> {
		Ok(IBtmDebug(self.0.duplicate()?))
	}
}

impl<T> Deref for IBtmDebug<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IBtmDebug<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IBtmDebug<T> {
	// fn unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown3(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown4(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown5(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown6(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown7(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown8(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IBtmDebug<T> {
	fn from(obj: T) -> IBtmDebug<T> {
		IBtmDebug(obj)
	}
}
