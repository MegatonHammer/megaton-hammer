
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IApplicationRootService<T>(T);

impl IApplicationRootService<Session> {
	pub fn new() -> Result<Arc<IApplicationRootService<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IApplicationRootService<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"vi:u\0\0\0\0") {
			let ret = Arc::new(IApplicationRootService(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"vi:u\0\0\0\0").map(|s: KObject| Arc::new(Session::from(s).into()));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}

	pub fn to_domain(self) -> ::core::result::Result<IApplicationRootService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IApplicationRootService(domain)),
			Err((sess, err)) => Err((IApplicationRootService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IApplicationRootService<Session>> {
		Ok(IApplicationRootService(self.0.duplicate()?))
	}
}

impl<T> Deref for IApplicationRootService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IApplicationRootService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IApplicationRootService<T> {
	pub fn get_display_service(&self, unk0: u32) -> Result<::nn::visrv::sf::IApplicationDisplayService<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

}

impl<T: Object> From<T> for IApplicationRootService<T> {
	fn from(obj: T) -> IApplicationRootService<T> {
		IApplicationRootService(obj)
	}
}
