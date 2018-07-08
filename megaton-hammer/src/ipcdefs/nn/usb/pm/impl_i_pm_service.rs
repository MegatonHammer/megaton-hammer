
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IPmService<T>(T);

impl IPmService<Session> {
	pub fn raw_new() -> Result<IPmService<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"usb:pm\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IPmService<Session>>> {
		use alloc::arc::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IPmService<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"usb:pm\0\0") {
			let ret = Arc::new(IPmService(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IPmService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IPmService(domain)),
			Err((sess, err)) => Err((IPmService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IPmService<Session>> {
		Ok(IPmService(self.0.duplicate()?))
	}
}

impl<T> Deref for IPmService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IPmService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IPmService<T> {
	pub fn unknown0(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown2(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn unknown3(&self, ) -> Result<u32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(3)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn unknown4(&self, unk0: u32, unk1: u32) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u32,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(4)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown5(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IPmService<T> {
	fn from(obj: T) -> IPmService<T> {
		IPmService(obj)
	}
}
