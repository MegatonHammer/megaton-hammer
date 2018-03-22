
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IHardwareOpusDecoderManager(Session);

impl IHardwareOpusDecoderManager {
	pub fn new() -> Result<Arc<IHardwareOpusDecoderManager>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IHardwareOpusDecoderManager>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"hwopus\0\0") {
			let ret = Arc::new(IHardwareOpusDecoderManager(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

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

	#[cfg(feature = "switch-3.0.0")]
	pub fn initialize_ex(&self, unk0: u32, unk1: &KObject, unk2: &[u8; 0x110]) -> Result<Session> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(unk0)
			.copy_handle(unk1)
			.descriptor(IPCBuffer::from_ref(unk2, 0x19))
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn get_work_buffer_size_ex(&self, unk0: &[u8; 0x110]) -> Result<u32> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			.descriptor(IPCBuffer::from_ref(unk0, 0x19))
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IHardwareOpusDecoderManager {
	unsafe fn from_kobject(obj: KObject) -> IHardwareOpusDecoderManager {
		IHardwareOpusDecoderManager(Session::from_kobject(obj))
	}
}
