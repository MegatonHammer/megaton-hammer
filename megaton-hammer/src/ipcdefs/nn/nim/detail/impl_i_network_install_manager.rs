
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::sync::Arc;

#[derive(Debug)]
pub struct INetworkInstallManager<T>(T);

impl INetworkInstallManager<Session> {
	pub fn raw_new() -> Result<INetworkInstallManager<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"nim\0\0\0\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<INetworkInstallManager<Session>>> {
		use alloc::sync::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<INetworkInstallManager<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"nim\0\0\0\0\0") {
			let ret = Arc::new(INetworkInstallManager(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<INetworkInstallManager<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(INetworkInstallManager(domain)),
			Err((sess, err)) => Err((INetworkInstallManager(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<INetworkInstallManager<Session>> {
		Ok(INetworkInstallManager(self.0.duplicate()?))
	}
}

impl<T> Deref for INetworkInstallManager<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for INetworkInstallManager<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> INetworkInstallManager<T> {
	pub fn unknown1(&self, unk0: u128) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for INetworkInstallManager<T> {
	fn from(obj: T) -> INetworkInstallManager<T> {
		INetworkInstallManager(obj)
	}
}
