
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::sync::Arc;

#[derive(Debug)]
pub struct IResolver<T>(T);

impl IResolver<Session> {
	pub fn raw_new() -> Result<IResolver<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"sfdnsres")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IResolver<Session>>> {
		use alloc::sync::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IResolver<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"sfdnsres") {
			let ret = Arc::new(IResolver(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IResolver<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IResolver(domain)),
			Err((sess, err)) => Err((IResolver(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IResolver<Session>> {
		Ok(IResolver(self.0.duplicate()?))
	}
}

impl<T> Deref for IResolver<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IResolver<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IResolver<T> {
	// fn set_dns_address_private(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_dns_address_private(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_host_by_name(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_host_by_addr(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_host_string_error(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_gai_string_error(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_addr_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_name_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn request_cancel_handle(&self, unk0: u64) -> Result<u32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(8)
			.args(unk0)
			.send_pid()
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn cancel_socket_call(&self, unk0: u32, unk1: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u64,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(9)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown10(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(10)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn clear_dns_ip_server_address_array(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(11)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IResolver<T> {
	fn from(obj: T) -> IResolver<T> {
		IResolver(obj)
	}
}
