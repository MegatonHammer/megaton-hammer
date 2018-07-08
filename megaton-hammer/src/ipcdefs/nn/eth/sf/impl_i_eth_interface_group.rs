
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IEthInterfaceGroup<T>(T);

impl IEthInterfaceGroup<Session> {
	pub fn raw_new() -> Result<IEthInterfaceGroup<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"ethc:i\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IEthInterfaceGroup<Session>>> {
		use alloc::arc::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IEthInterfaceGroup<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"ethc:i\0\0") {
			let ret = Arc::new(IEthInterfaceGroup(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IEthInterfaceGroup<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IEthInterfaceGroup(domain)),
			Err((sess, err)) => Err((IEthInterfaceGroup(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IEthInterfaceGroup<Session>> {
		Ok(IEthInterfaceGroup(self.0.duplicate()?))
	}
}

impl<T> Deref for IEthInterfaceGroup<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IEthInterfaceGroup<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IEthInterfaceGroup<T> {
	pub fn unknown0(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn unknown1(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown2(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown3(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown4(&self, ) -> Result<u32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(4)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl<T: Object> From<T> for IEthInterfaceGroup<T> {
	fn from(obj: T) -> IEthInterfaceGroup<T> {
		IEthInterfaceGroup(obj)
	}
}
