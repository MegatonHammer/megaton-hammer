
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct ISslService<T>(T);

impl ISslService<Session> {
	pub fn raw_new() -> Result<ISslService<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"ssl\0\0\0\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<ISslService<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<ISslService<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"ssl\0\0\0\0\0") {
			let ret = Arc::new(ISslService(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<ISslService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ISslService(domain)),
			Err((sess, err)) => Err((ISslService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ISslService<Session>> {
		Ok(ISslService(self.0.duplicate()?))
	}
}

impl<T> Deref for ISslService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ISslService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ISslService<T> {
	pub fn create_context(&self, unk0: ::ipcdefs::nn::ssl::sf::SslVersion, unk1: u64) -> Result<::ipcdefs::nn::ssl::sf::ISslContext<T>> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::ssl::sf::SslVersion,
			unk1: u64,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_context_count(&self, ) -> Result<u32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_certificates(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_certificate_buf_size(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn debug_ioctl(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn set_interface_version(&self, unk0: u32) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(5)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for ISslService<T> {
	fn from(obj: T) -> ISslService<T> {
		ISslService(obj)
	}
}
