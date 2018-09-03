
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::sync::Arc;

#[derive(Debug)]
pub struct IFinalOutputRecorderManager<T>(T);

impl IFinalOutputRecorderManager<Session> {
	pub fn raw_new() -> Result<IFinalOutputRecorderManager<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"audren:u")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IFinalOutputRecorderManager<Session>>> {
		use alloc::sync::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IFinalOutputRecorderManager<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"audren:u") {
			let ret = Arc::new(IFinalOutputRecorderManager(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IFinalOutputRecorderManager<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IFinalOutputRecorderManager(domain)),
			Err((sess, err)) => Err((IFinalOutputRecorderManager(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IFinalOutputRecorderManager<Session>> {
		Ok(IFinalOutputRecorderManager(self.0.duplicate()?))
	}
}

impl<T> Deref for IFinalOutputRecorderManager<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IFinalOutputRecorderManager<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IFinalOutputRecorderManager<T> {
	pub fn open_final_output_recorder(&self, unk0: u64, unk1: u64, unk2: &KObject) -> Result<(u128, ::ipcdefs::nn::audio::detail::IFinalOutputRecorder<T>)> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u64,
		}
		let req : Request<_, [_; 0], [_; 1], [_; 0]> = Request::new(0)
			.args(InRaw {
				unk0,
				unk1,
			})
			.copy_handle(unk2)
			;
		let mut res : Response<u128> = self.0.send(req)?;
		Ok((*res.get_raw(),T::from_res(&mut res).into()))
	}

}

impl<T: Object> From<T> for IFinalOutputRecorderManager<T> {
	fn from(obj: T) -> IFinalOutputRecorderManager<T> {
		IFinalOutputRecorderManager(obj)
	}
}
