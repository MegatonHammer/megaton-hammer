
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct ILogService<T>(T);

impl ILogService<Session> {
	pub fn raw_new() -> Result<ILogService<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"lm\0\0\0\0\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<ILogService<Session>>> {
		use alloc::arc::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<ILogService<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"lm\0\0\0\0\0\0") {
			let ret = Arc::new(ILogService(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<ILogService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ILogService(domain)),
			Err((sess, err)) => Err((ILogService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ILogService<Session>> {
		Ok(ILogService(self.0.duplicate()?))
	}
}

impl<T> Deref for ILogService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ILogService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ILogService<T> {
	pub fn open_logger(&self, unk0: u64) -> Result<::ipcdefs::nn::lm::ILogger<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

}

impl<T: Object> From<T> for ILogService<T> {
	fn from(obj: T) -> ILogService<T> {
		ILogService(obj)
	}
}
