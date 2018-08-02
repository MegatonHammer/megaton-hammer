
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::sync::Arc;

#[derive(Debug)]
pub struct IFinalOutputRecorderManagerForDebugger<T>(T);

impl IFinalOutputRecorderManagerForDebugger<Session> {
	pub fn raw_new() -> Result<IFinalOutputRecorderManagerForDebugger<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"audrec:d")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IFinalOutputRecorderManagerForDebugger<Session>>> {
		use alloc::sync::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IFinalOutputRecorderManagerForDebugger<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"audrec:d") {
			let ret = Arc::new(IFinalOutputRecorderManagerForDebugger(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IFinalOutputRecorderManagerForDebugger<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IFinalOutputRecorderManagerForDebugger(domain)),
			Err((sess, err)) => Err((IFinalOutputRecorderManagerForDebugger(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IFinalOutputRecorderManagerForDebugger<Session>> {
		Ok(IFinalOutputRecorderManagerForDebugger(self.0.duplicate()?))
	}
}

impl<T> Deref for IFinalOutputRecorderManagerForDebugger<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IFinalOutputRecorderManagerForDebugger<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IFinalOutputRecorderManagerForDebugger<T> {
	pub fn unknown0(&self, unk0: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown1(&self, unk0: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IFinalOutputRecorderManagerForDebugger<T> {
	fn from(obj: T) -> IFinalOutputRecorderManagerForDebugger<T> {
		IFinalOutputRecorderManagerForDebugger(obj)
	}
}
