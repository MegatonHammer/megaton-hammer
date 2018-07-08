
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IManagerInterface<T>(T);

impl IManagerInterface<Session> {
	pub fn raw_new() -> Result<IManagerInterface<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"sm:m\0\0\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IManagerInterface<Session>>> {
		use alloc::arc::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IManagerInterface<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"sm:m\0\0\0\0") {
			let ret = Arc::new(IManagerInterface(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IManagerInterface<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IManagerInterface(domain)),
			Err((sess, err)) => Err((IManagerInterface(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IManagerInterface<Session>> {
		Ok(IManagerInterface(self.0.duplicate()?))
	}
}

impl<T> Deref for IManagerInterface<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IManagerInterface<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IManagerInterface<T> {
	// fn register_process(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unregister_process(&self, unk0: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IManagerInterface<T> {
	fn from(obj: T) -> IManagerInterface<T> {
		IManagerInterface(obj)
	}
}
