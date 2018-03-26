
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IBootModeInterface<T>(T);

impl IBootModeInterface<Session> {
	pub fn new() -> Result<Arc<IBootModeInterface<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IBootModeInterface<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"pm:bm\0\0\0") {
			let ret = Arc::new(IBootModeInterface(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"pm:bm\0\0\0").map(|s: KObject| Arc::new(Session::from(s).into()));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}

	pub fn to_domain(self) -> ::core::result::Result<IBootModeInterface<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IBootModeInterface(domain)),
			Err((sess, err)) => Err((IBootModeInterface(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IBootModeInterface<Session>> {
		Ok(IBootModeInterface(self.0.duplicate()?))
	}
}

impl<T> Deref for IBootModeInterface<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IBootModeInterface<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IBootModeInterface<T> {
	pub fn get_boot_mode(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_maintenance_boot(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IBootModeInterface<T> {
	fn from(obj: T) -> IBootModeInterface<T> {
		IBootModeInterface(obj)
	}
}
