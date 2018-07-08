
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct INotifyService<T>(T);

impl INotifyService<Session> {
	pub fn raw_new() -> Result<INotifyService<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"pdm:ntfy")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<INotifyService<Session>>> {
		use alloc::arc::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<INotifyService<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"pdm:ntfy") {
			let ret = Arc::new(INotifyService(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<INotifyService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(INotifyService(domain)),
			Err((sess, err)) => Err((INotifyService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<INotifyService<Session>> {
		Ok(INotifyService(self.0.duplicate()?))
	}
}

impl<T> Deref for INotifyService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for INotifyService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> INotifyService<T> {
	pub fn unknown0(&self, unk0: u64, unk1: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u64,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown2(&self, unk0: u8) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown3(&self, unk0: u8) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(3)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown4(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(4)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown5(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for INotifyService<T> {
	fn from(obj: T) -> INotifyService<T> {
		INotifyService(obj)
	}
}
