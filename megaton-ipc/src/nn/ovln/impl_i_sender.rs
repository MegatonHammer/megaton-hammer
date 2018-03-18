
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct ISender(Session);

impl ISender {
	pub fn new() -> Result<Arc<ISender>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<ISender>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"ovln:snd") {
			let ret = Arc::new(ISender(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"ovln:snd").map(|s| Arc::new(unsafe { ISender::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for ISender {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ISender {
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
		let req = Request::new(0)
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

impl FromKObject for ISender {
	unsafe fn from_kobject(obj: KObject) -> ISender {
		ISender(Session::from_kobject(obj))
	}
}
