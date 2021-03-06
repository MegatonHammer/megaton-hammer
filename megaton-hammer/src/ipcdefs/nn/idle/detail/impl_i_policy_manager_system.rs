
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::sync::Arc;

#[derive(Debug)]
pub struct IPolicyManagerSystem<T>(T);

impl IPolicyManagerSystem<Session> {
	pub fn raw_new() -> Result<IPolicyManagerSystem<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"idle:sys")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IPolicyManagerSystem<Session>>> {
		use alloc::sync::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IPolicyManagerSystem<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"idle:sys") {
			let ret = Arc::new(IPolicyManagerSystem(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IPolicyManagerSystem<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IPolicyManagerSystem(domain)),
			Err((sess, err)) => Err((IPolicyManagerSystem(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IPolicyManagerSystem<Session>> {
		Ok(IPolicyManagerSystem(self.0.duplicate()?))
	}
}

impl<T> Deref for IPolicyManagerSystem<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IPolicyManagerSystem<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IPolicyManagerSystem<T> {
	pub fn get_auto_power_down_event(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	#[cfg(not(feature = "switch-4.0.0"))]
	pub fn unknown1(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(not(feature = "switch-4.0.0"))]
	pub fn unknown2(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown3(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown4(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(4)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown5(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(5)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IPolicyManagerSystem<T> {
	fn from(obj: T) -> IPolicyManagerSystem<T> {
		IPolicyManagerSystem(obj)
	}
}
