
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::sync::Arc;

#[derive(Debug)]
pub struct IHtcManager<T>(T);

impl IHtcManager<Session> {
	pub fn raw_new() -> Result<IHtcManager<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"htc\0\0\0\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IHtcManager<Session>>> {
		use alloc::sync::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IHtcManager<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"htc\0\0\0\0\0") {
			let ret = Arc::new(IHtcManager(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IHtcManager<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IHtcManager(domain)),
			Err((sess, err)) => Err((IHtcManager(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IHtcManager<Session>> {
		Ok(IHtcManager(self.0.duplicate()?))
	}
}

impl<T> Deref for IHtcManager<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IHtcManager<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IHtcManager<T> {
	// fn get_environment_variable(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_environment_variable_length(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn bind_host_connection_event(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn bind_host_disconnection_event(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(3)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn bind_host_connection_event_for_system(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn bind_host_disconnection_event_for_system(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(5)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn get_bridge_ip_address(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_bridge_port(&self, UNKNOWN) -> Result<UNKNOWN>;
	#[cfg(feature = "switch-3.0.0")]
	pub fn set_usb_detached_for_debug(&self, unk0: u8) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(8)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_bridge_subnet_mask(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_bridge_mac_address(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IHtcManager<T> {
	fn from(obj: T) -> IHtcManager<T> {
		IHtcManager(obj)
	}
}
