
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::sync::Arc;

#[derive(Debug)]
pub struct IStaticService<T>(T);

impl IStaticService<Session> {
	pub fn raw_new_time_s() -> Result<IStaticService<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"time:s\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new_time_s() -> Result<Arc<IStaticService<Session>>> {
		use alloc::sync::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IStaticService<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"time:s\0\0") {
			let ret = Arc::new(IStaticService(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new_time_s()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn raw_new_time_u() -> Result<IStaticService<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"time:u\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new_time_u() -> Result<Arc<IStaticService<Session>>> {
		use alloc::sync::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IStaticService<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"time:u\0\0") {
			let ret = Arc::new(IStaticService(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new_time_u()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn raw_new_time_r() -> Result<IStaticService<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"time:r\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new_time_r() -> Result<Arc<IStaticService<Session>>> {
		use alloc::sync::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IStaticService<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"time:r\0\0") {
			let ret = Arc::new(IStaticService(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new_time_r()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn raw_new_time_a() -> Result<IStaticService<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"time:a\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new_time_a() -> Result<Arc<IStaticService<Session>>> {
		use alloc::sync::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IStaticService<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"time:a\0\0") {
			let ret = Arc::new(IStaticService(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new_time_a()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IStaticService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IStaticService(domain)),
			Err((sess, err)) => Err((IStaticService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IStaticService<Session>> {
		Ok(IStaticService(self.0.duplicate()?))
	}
}

impl<T> Deref for IStaticService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IStaticService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IStaticService<T> {
	pub fn get_standard_user_system_clock(&self, ) -> Result<::ipcdefs::nn::timesrv::detail::service::ISystemClock<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_standard_network_system_clock(&self, ) -> Result<::ipcdefs::nn::timesrv::detail::service::ISystemClock<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_standard_steady_clock(&self, ) -> Result<::ipcdefs::nn::timesrv::detail::service::ISteadyClock<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_time_zone_service(&self, ) -> Result<::ipcdefs::nn::timesrv::detail::service::ITimeZoneService<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(3)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_standard_local_system_clock(&self, ) -> Result<::ipcdefs::nn::timesrv::detail::service::ISystemClock<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	// fn get_ephemeral_network_system_clock(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_standard_steady_clock_internal_offset(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn is_standard_user_system_clock_automatic_correction_enabled(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(100)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_standard_user_system_clock_automatic_correction_enabled(&self, unk0: bool) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(101)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_standard_user_system_clock_initial_year(&self, UNKNOWN) -> Result<UNKNOWN>;
	#[cfg(feature = "switch-3.0.0")]
	pub fn is_standard_network_system_clock_accuracy_sufficient(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(200)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn calculate_monotonic_system_clock_base_time_point(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_clock_snapshot(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_clock_snapshot_from_system_clock_context(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn calculate_standard_user_system_clock_difference_by_user(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn calculate_span_between(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IStaticService<T> {
	fn from(obj: T) -> IStaticService<T> {
		IStaticService(obj)
	}
}
