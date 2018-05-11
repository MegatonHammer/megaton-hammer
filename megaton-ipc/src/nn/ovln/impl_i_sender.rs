
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct ISender<T>(T);

impl ISender<Session> {
	pub fn raw_new() -> Result<ISender<Session>> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		let r = sm.get_service(*b"ovln:snd").map(|s: KObject| Session::from(s).into());
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}

	pub fn new() -> Result<Arc<ISender<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<ISender<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"ovln:snd") {
			let ret = Arc::new(ISender(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<ISender<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ISender(domain)),
			Err((sess, err)) => Err((ISender(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ISender<Session>> {
		Ok(ISender(self.0.duplicate()?))
	}
}

impl<T> Deref for ISender<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ISender<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ISender<T> {
	pub fn unknown0(&self, unk1: u64, unk2: u64, unk3: u64, unk4: u64, unk5: u64, unk6: u64, unk7: u64, unk8: u64, unk9: u64, unk10: u64, unk11: u64, unk12: u64, unk13: u64, unk14: u64, unk15: u64, unk16: u64, unk17: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk1: u64,
			unk2: u64,
			unk3: u64,
			unk4: u64,
			unk5: u64,
			unk6: u64,
			unk7: u64,
			unk8: u64,
			unk9: u64,
			unk10: u64,
			unk11: u64,
			unk12: u64,
			unk13: u64,
			unk14: u64,
			unk15: u64,
			unk16: u64,
			unk17: u64,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(InRaw {
				unk1,
				unk2,
				unk3,
				unk4,
				unk5,
				unk6,
				unk7,
				unk8,
				unk9,
				unk10,
				unk11,
				unk12,
				unk13,
				unk14,
				unk15,
				unk16,
				unk17,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for ISender<T> {
	fn from(obj: T) -> ISender<T> {
		ISender(obj)
	}
}
