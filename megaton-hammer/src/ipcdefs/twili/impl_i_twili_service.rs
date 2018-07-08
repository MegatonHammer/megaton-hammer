
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct ITwiliService<T>(T);

impl ITwiliService<Session> {
	pub fn raw_new() -> Result<ITwiliService<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"twili\0\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<ITwiliService<Session>>> {
		use alloc::arc::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<ITwiliService<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"twili\0\0\0") {
			let ret = Arc::new(ITwiliService(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<ITwiliService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ITwiliService(domain)),
			Err((sess, err)) => Err((ITwiliService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ITwiliService<Session>> {
		Ok(ITwiliService(self.0.duplicate()?))
	}
}

impl<T> Deref for ITwiliService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ITwiliService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ITwiliService<T> {
	pub fn open_stdin(&self, ) -> Result<::ipcdefs::twili::IPipe<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn open_stdout(&self, ) -> Result<::ipcdefs::twili::IPipe<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn open_stderr(&self, ) -> Result<::ipcdefs::twili::IPipe<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn open_hbabi_shim(&self, ) -> Result<::ipcdefs::twili::IHBABIShim<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(3)
			.args(())
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn destroy(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(999)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for ITwiliService<T> {
	fn from(obj: T) -> ITwiliService<T> {
		ITwiliService(obj)
	}
}
