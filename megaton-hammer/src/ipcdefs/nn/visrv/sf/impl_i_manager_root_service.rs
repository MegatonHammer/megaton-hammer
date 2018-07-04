
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IManagerRootService<T>(T);

impl IManagerRootService<Session> {
	pub fn raw_new() -> Result<IManagerRootService<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"vi:m\0\0\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IManagerRootService<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IManagerRootService<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"vi:m\0\0\0\0") {
			let ret = Arc::new(IManagerRootService(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IManagerRootService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IManagerRootService(domain)),
			Err((sess, err)) => Err((IManagerRootService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IManagerRootService<Session>> {
		Ok(IManagerRootService(self.0.duplicate()?))
	}
}

impl<T> Deref for IManagerRootService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IManagerRootService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IManagerRootService<T> {
	pub fn get_display_service(&self, unk0: u32) -> Result<::ipcdefs::nn::visrv::sf::IApplicationDisplayService<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_display_service_with_proxy_name_exchange(&self, unk0: ::ipcdefs::nn::vi::ProxyName, unk1: u32) -> Result<::ipcdefs::nn::visrv::sf::IApplicationDisplayService<T>> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::vi::ProxyName,
			unk1: u32,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(3)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

}

impl<T: Object> From<T> for IManagerRootService<T> {
	fn from(obj: T) -> IManagerRootService<T> {
		IManagerRootService(obj)
	}
}
