
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IBtmSystem<T>(T);

impl IBtmSystem<Session> {
	pub fn raw_new() -> Result<IBtmSystem<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"btm:sys\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IBtmSystem<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IBtmSystem<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"btm:sys\0") {
			let ret = Arc::new(IBtmSystem(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IBtmSystem<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IBtmSystem(domain)),
			Err((sess, err)) => Err((IBtmSystem(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IBtmSystem<Session>> {
		Ok(IBtmSystem(self.0.duplicate()?))
	}
}

impl<T> Deref for IBtmSystem<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IBtmSystem<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IBtmSystem<T> {
	pub fn get_core_impl(&self, ) -> Result<T> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

}

impl<T: Object> From<T> for IBtmSystem<T> {
	fn from(obj: T) -> IBtmSystem<T> {
		IBtmSystem(obj)
	}
}
