
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct ILdrShellInterface(Session);

impl ILdrShellInterface {
	pub fn new() -> Result<Arc<ILdrShellInterface>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<ILdrShellInterface>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"ldr:shel") {
			let ret = Arc::new(ILdrShellInterface(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"ldr:shel").map(|s| Arc::new(unsafe { ILdrShellInterface::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for ILdrShellInterface {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ILdrShellInterface {
	pub fn add_process_to_launch_queue(&self, unk0: &[u8; 0x200], size: u32, app_id: ::nn::ncm::ApplicationId) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			size: u32,
			app_id: ::nn::ncm::ApplicationId,
		}
		let req = Request::new(0)
			.args(InRaw {
				size,
				app_id,
			})
			.descriptor(IPCBuffer::from_ref(unk0, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn clear_launch_queue(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for ILdrShellInterface {
	unsafe fn from_kobject(obj: KObject) -> ILdrShellInterface {
		ILdrShellInterface(Session::from_kobject(obj))
	}
}
