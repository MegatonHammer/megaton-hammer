
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::sync::Arc;

#[derive(Debug)]
pub struct ServerInterface<T>(T);

impl ServerInterface<Session> {
	pub fn raw_new() -> Result<ServerInterface<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"bsdcfg\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<ServerInterface<Session>>> {
		use alloc::sync::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<ServerInterface<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"bsdcfg\0\0") {
			let ret = Arc::new(ServerInterface(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<ServerInterface<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ServerInterface(domain)),
			Err((sess, err)) => Err((ServerInterface(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ServerInterface<Session>> {
		Ok(ServerInterface(self.0.duplicate()?))
	}
}

impl<T> Deref for ServerInterface<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ServerInterface<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ServerInterface<T> {
	// fn set_if_up(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_if_up_with_event(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn cancel_if(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_if_down(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_if_state(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn dhcp_renew(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn add_static_arp_entry(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn remove_arp_entry(&self, unk0: u32) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(7)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn lookup_arp_entry(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn lookup_arp_entry2(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn clear_arp_entries(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(10)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn clear_arp_entries2(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn print_arp_entries(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(12)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for ServerInterface<T> {
	fn from(obj: T) -> ServerInterface<T> {
		ServerInterface(obj)
	}
}
