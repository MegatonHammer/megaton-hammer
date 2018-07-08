
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IAudioRendererManagerForDebugger<T>(T);

impl IAudioRendererManagerForDebugger<Session> {
	pub fn raw_new() -> Result<IAudioRendererManagerForDebugger<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"audren:d")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IAudioRendererManagerForDebugger<Session>>> {
		use alloc::arc::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IAudioRendererManagerForDebugger<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"audren:d") {
			let ret = Arc::new(IAudioRendererManagerForDebugger(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IAudioRendererManagerForDebugger<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IAudioRendererManagerForDebugger(domain)),
			Err((sess, err)) => Err((IAudioRendererManagerForDebugger(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IAudioRendererManagerForDebugger<Session>> {
		Ok(IAudioRendererManagerForDebugger(self.0.duplicate()?))
	}
}

impl<T> Deref for IAudioRendererManagerForDebugger<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IAudioRendererManagerForDebugger<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IAudioRendererManagerForDebugger<T> {
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

impl<T: Object> From<T> for IAudioRendererManagerForDebugger<T> {
	fn from(obj: T) -> IAudioRendererManagerForDebugger<T> {
		IAudioRendererManagerForDebugger(obj)
	}
}
