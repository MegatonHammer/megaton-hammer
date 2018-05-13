
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IStaticService<T>(T);

impl IStaticService<Session> {
	pub fn raw_new_mii_e() -> Result<IStaticService<Session>> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"mii:e\0\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new_mii_e() -> Result<Arc<IStaticService<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IStaticService<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"mii:e\0\0\0") {
			let ret = Arc::new(IStaticService(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new_mii_e()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn raw_new_mii_u() -> Result<IStaticService<Session>> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"mii:u\0\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new_mii_u() -> Result<Arc<IStaticService<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IStaticService<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"mii:u\0\0\0") {
			let ret = Arc::new(IStaticService(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new_mii_u()?;
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
	pub fn get_database_service_shared_pointer(&self, unk0: i32) -> Result<::nn::mii::detail::IDatabaseService<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

}

impl<T: Object> From<T> for IStaticService<T> {
	fn from(obj: T) -> IStaticService<T> {
		IStaticService(obj)
	}
}
