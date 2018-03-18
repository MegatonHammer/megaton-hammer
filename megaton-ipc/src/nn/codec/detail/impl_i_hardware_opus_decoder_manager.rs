
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IHardwareOpusDecoderManager(Session);

impl IHardwareOpusDecoderManager {
	pub fn new() -> Result<Arc<IHardwareOpusDecoderManager>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IHardwareOpusDecoderManager>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		let r = sm.get_service(*b"hwopus\0\0").map(|s| Arc::new(unsafe { IHardwareOpusDecoderManager::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IHardwareOpusDecoderManager {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IHardwareOpusDecoderManager {
	pub fn initialize(&self, unk0: u64, unk1: u32, unk2: &KObject) -> Result<Session> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u32,
		}
		let req = Request::new(0)
			.args(InRaw {
				unk0,
				unk1,
			})
			.copy_handle(unk2)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn get_work_buffer_size(&self, unk0: u64) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(unk0)
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn initialize_ex(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_work_buffer_size_ex(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IHardwareOpusDecoderManager {
	unsafe fn from_kobject(obj: KObject) -> IHardwareOpusDecoderManager {
		IHardwareOpusDecoderManager(Session::from_kobject(obj))
	}
}
