
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::sync::Arc;

#[derive(Debug)]
pub struct IPrepoService<T>(T);

impl IPrepoService<Session> {
	pub fn raw_new_prepo_a() -> Result<IPrepoService<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"prepo:a\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new_prepo_a() -> Result<Arc<IPrepoService<Session>>> {
		use alloc::sync::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IPrepoService<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"prepo:a\0") {
			let ret = Arc::new(IPrepoService(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new_prepo_a()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn raw_new_prepo_m() -> Result<IPrepoService<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"prepo:m\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new_prepo_m() -> Result<Arc<IPrepoService<Session>>> {
		use alloc::sync::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IPrepoService<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"prepo:m\0") {
			let ret = Arc::new(IPrepoService(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new_prepo_m()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn raw_new_prepo_u() -> Result<IPrepoService<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"prepo:u\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new_prepo_u() -> Result<Arc<IPrepoService<Session>>> {
		use alloc::sync::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IPrepoService<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"prepo:u\0") {
			let ret = Arc::new(IPrepoService(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new_prepo_u()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn raw_new_prepo_s() -> Result<IPrepoService<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"prepo:s\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new_prepo_s() -> Result<Arc<IPrepoService<Session>>> {
		use alloc::sync::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IPrepoService<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"prepo:s\0") {
			let ret = Arc::new(IPrepoService(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new_prepo_s()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IPrepoService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IPrepoService(domain)),
			Err((sess, err)) => Err((IPrepoService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IPrepoService<Session>> {
		Ok(IPrepoService(self.0.duplicate()?))
	}
}

impl<T> Deref for IPrepoService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IPrepoService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IPrepoService<T> {
	// fn save_report(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn save_report_with_user(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn request_immediate_transmission(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(10200)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_transmission_status(&self, ) -> Result<i32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(10300)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn save_system_report(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn save_system_report_with_user(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_operation_mode(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn clear_storage(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30100)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_user_agreement_check_enabled(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(40100)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_user_agreement_check_enabled(&self, unk0: bool) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(40101)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_storage_usage(&self, ) -> Result<(i64, i64)> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(90100)
			.args(())
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk0: i64,
			unk1: i64,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk0.clone(),res.get_raw().unk1.clone()))
	}

	// fn get_statistics(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_throughput_history(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_last_upload_error(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IPrepoService<T> {
	fn from(obj: T) -> IPrepoService<T> {
		IPrepoService(obj)
	}
}
