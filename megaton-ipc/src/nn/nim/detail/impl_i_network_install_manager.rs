
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct INetworkInstallManager<T>(T);

impl INetworkInstallManager<Session> {
	pub fn new() -> Result<Arc<INetworkInstallManager<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<INetworkInstallManager<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"nim\0\0\0\0\0") {
			let ret = Arc::new(INetworkInstallManager(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"nim\0\0\0\0\0").map(|s: KObject| Arc::new(Session::from(s).into()));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
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
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
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
