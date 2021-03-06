
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::sync::Arc;

#[derive(Debug)]
pub struct IFinalOutputRecorderManagerForApplet<T>(T);

impl IFinalOutputRecorderManagerForApplet<Session> {
	pub fn raw_new() -> Result<IFinalOutputRecorderManagerForApplet<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"audrec:a")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IFinalOutputRecorderManagerForApplet<Session>>> {
		use alloc::sync::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IFinalOutputRecorderManagerForApplet<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"audrec:a") {
			let ret = Arc::new(IFinalOutputRecorderManagerForApplet(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IFinalOutputRecorderManagerForApplet<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IFinalOutputRecorderManagerForApplet(domain)),
			Err((sess, err)) => Err((IFinalOutputRecorderManagerForApplet(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IFinalOutputRecorderManagerForApplet<Session>> {
		Ok(IFinalOutputRecorderManagerForApplet(self.0.duplicate()?))
	}
}

impl<T> Deref for IFinalOutputRecorderManagerForApplet<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IFinalOutputRecorderManagerForApplet<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IFinalOutputRecorderManagerForApplet<T> {
	pub fn unknown0(&self, unk0: u64, unk1: u64) -> Result<KObject> {
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
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn unknown1(&self, unk0: u64, unk1: u64) -> Result<KObject> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u64,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

}

impl<T: Object> From<T> for IFinalOutputRecorderManagerForApplet<T> {
	fn from(obj: T) -> IFinalOutputRecorderManagerForApplet<T> {
		IFinalOutputRecorderManagerForApplet(obj)
	}
}
