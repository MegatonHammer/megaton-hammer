
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IAudioOutManager<T>(T);

impl IAudioOutManager<Session> {
	pub fn new() -> Result<Arc<IAudioOutManager<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IAudioOutManager<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"audout:u") {
			let ret = Arc::new(IAudioOutManager(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"audout:u").map(|s: KObject| Arc::new(Session::from(s).into()));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}

	pub fn to_domain(self) -> ::core::result::Result<IAudioOutManager<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IAudioOutManager(domain)),
			Err((sess, err)) => Err((IAudioOutManager(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IAudioOutManager<Session>> {
		Ok(IAudioOutManager(self.0.duplicate()?))
	}
}

impl<T> Deref for IAudioOutManager<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IAudioOutManager<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IAudioOutManager<T> {
	// fn list_audio_outs(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn open_audio_out(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn list_audio_outs_ex(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn open_audio_out_ex(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IAudioOutManager<T> {
	fn from(obj: T) -> IAudioOutManager<T> {
		IAudioOutManager(obj)
	}
}
