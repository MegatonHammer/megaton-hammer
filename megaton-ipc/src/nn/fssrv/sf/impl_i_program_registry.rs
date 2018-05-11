
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IProgramRegistry<T>(T);

impl IProgramRegistry<Session> {
	pub fn raw_new() -> Result<IProgramRegistry<Session>> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let r = sm.get_service(*b"fsp-pr\0\0").map(|s: KObject| Session::from(s).into());
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}

	pub fn new() -> Result<Arc<IProgramRegistry<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IProgramRegistry<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"fsp-pr\0\0") {
			let ret = Arc::new(IProgramRegistry(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IProgramRegistry<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IProgramRegistry(domain)),
			Err((sess, err)) => Err((IProgramRegistry(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IProgramRegistry<Session>> {
		Ok(IProgramRegistry(self.0.duplicate()?))
	}
}

impl<T> Deref for IProgramRegistry<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IProgramRegistry<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IProgramRegistry<T> {
	// fn set_fs_permissions(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn clear_fs_permissions(&self, pid: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(pid)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_enabled_program_verification(&self, enabled: u8) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(256)
			.args(enabled)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IProgramRegistry<T> {
	fn from(obj: T) -> IProgramRegistry<T> {
		IProgramRegistry(obj)
	}
}
