
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IScreenShotControlService<T>(T);

impl IScreenShotControlService<Session> {
	pub fn raw_new() -> Result<IScreenShotControlService<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"caps:sc\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IScreenShotControlService<Session>>> {
		use alloc::arc::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IScreenShotControlService<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"caps:sc\0") {
			let ret = Arc::new(IScreenShotControlService(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IScreenShotControlService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IScreenShotControlService(domain)),
			Err((sess, err)) => Err((IScreenShotControlService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IScreenShotControlService<Session>> {
		Ok(IScreenShotControlService(self.0.duplicate()?))
	}
}

impl<T> Deref for IScreenShotControlService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IScreenShotControlService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IScreenShotControlService<T> {
	// fn unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown1001(&self, unk0: u64, unk1: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u64,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1001)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown1002(&self, unk0: u64, unk1: u64, unk2: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u64,
			unk2: u64,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1002)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown1003(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown1011(&self, unk0: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1011)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown1012(&self, unk0: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1012)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown1201(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown1202(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1202)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown1203(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IScreenShotControlService<T> {
	fn from(obj: T) -> IScreenShotControlService<T> {
		IScreenShotControlService(obj)
	}
}
