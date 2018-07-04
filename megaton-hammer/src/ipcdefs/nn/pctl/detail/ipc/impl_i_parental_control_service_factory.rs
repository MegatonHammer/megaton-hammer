
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IParentalControlServiceFactory<T>(T);

impl IParentalControlServiceFactory<Session> {
	pub fn raw_new_pctl_s() -> Result<IParentalControlServiceFactory<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"pctl:s\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new_pctl_s() -> Result<Arc<IParentalControlServiceFactory<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IParentalControlServiceFactory<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"pctl:s\0\0") {
			let ret = Arc::new(IParentalControlServiceFactory(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new_pctl_s()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn raw_new_pctl_r() -> Result<IParentalControlServiceFactory<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"pctl:r\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new_pctl_r() -> Result<Arc<IParentalControlServiceFactory<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IParentalControlServiceFactory<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"pctl:r\0\0") {
			let ret = Arc::new(IParentalControlServiceFactory(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new_pctl_r()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn raw_new_pctl_a() -> Result<IParentalControlServiceFactory<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"pctl:a\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new_pctl_a() -> Result<Arc<IParentalControlServiceFactory<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IParentalControlServiceFactory<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"pctl:a\0\0") {
			let ret = Arc::new(IParentalControlServiceFactory(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new_pctl_a()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn raw_new_pctl() -> Result<IParentalControlServiceFactory<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"pctl\0\0\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new_pctl() -> Result<Arc<IParentalControlServiceFactory<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IParentalControlServiceFactory<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"pctl\0\0\0\0") {
			let ret = Arc::new(IParentalControlServiceFactory(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new_pctl()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IParentalControlServiceFactory<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IParentalControlServiceFactory(domain)),
			Err((sess, err)) => Err((IParentalControlServiceFactory(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IParentalControlServiceFactory<Session>> {
		Ok(IParentalControlServiceFactory(self.0.duplicate()?))
	}
}

impl<T> Deref for IParentalControlServiceFactory<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IParentalControlServiceFactory<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IParentalControlServiceFactory<T> {
	pub fn get_service(&self, unk0: u64) -> Result<::ipcdefs::nn::pctl::detail::ipc::IParentalControlService<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

}

impl<T: Object> From<T> for IParentalControlServiceFactory<T> {
	fn from(obj: T) -> IParentalControlServiceFactory<T> {
		IParentalControlServiceFactory(obj)
	}
}
