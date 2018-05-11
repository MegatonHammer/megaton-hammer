
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IFinalOutputRecorderManagerForApplet<T>(T);

impl IFinalOutputRecorderManagerForApplet<Session> {
	pub fn raw_new() -> Result<IFinalOutputRecorderManagerForApplet<Session>> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		let r = sm.get_service(*b"audrec:a").map(|s: KObject| Session::from(s).into());
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}

	pub fn new() -> Result<Arc<IFinalOutputRecorderManagerForApplet<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IFinalOutputRecorderManagerForApplet<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"audrec:a") {
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
	pub fn request_suspend_final_output_recorders(&self, unk0: u64, unk1: u64) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

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

	pub fn request_resume_final_output_recorders(&self, unk0: u64, unk1: u64) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

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
