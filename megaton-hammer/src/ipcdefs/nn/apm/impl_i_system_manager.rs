
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::sync::Arc;

#[derive(Debug)]
pub struct ISystemManager<T>(T);

impl ISystemManager<Session> {
	pub fn raw_new() -> Result<ISystemManager<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"apm:sys\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<ISystemManager<Session>>> {
		use alloc::sync::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<ISystemManager<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"apm:sys\0") {
			let ret = Arc::new(ISystemManager(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<ISystemManager<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ISystemManager(domain)),
			Err((sess, err)) => Err((ISystemManager(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ISystemManager<Session>> {
		Ok(ISystemManager(self.0.duplicate()?))
	}
}

impl<T> Deref for ISystemManager<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ISystemManager<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ISystemManager<T> {
	pub fn request_performance_mode(&self, unk0: ::ipcdefs::nn::apm::PerformanceMode) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_performance_event(&self, unk0: ::ipcdefs::nn::apm::EventTarget) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn get_throttling_state(&self, ) -> Result<::ipcdefs::nn::apm::ThrottlingState> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let res : Response<::ipcdefs::nn::apm::ThrottlingState> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_last_throttling_state(&self, ) -> Result<::ipcdefs::nn::apm::ThrottlingState> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(3)
			.args(())
			;
		let res : Response<::ipcdefs::nn::apm::ThrottlingState> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn clear_last_throttling_state(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(4)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn load_and_apply_settings(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for ISystemManager<T> {
	fn from(obj: T) -> ISystemManager<T> {
		ISystemManager(obj)
	}
}
