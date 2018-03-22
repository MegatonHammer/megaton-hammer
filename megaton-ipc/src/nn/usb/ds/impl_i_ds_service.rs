
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IDsService(Session);

impl IDsService {
	pub fn new() -> Result<Arc<IDsService>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IDsService>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"usb:ds\0\0") {
			let ret = Arc::new(IDsService(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"usb:ds\0\0").map(|s| Arc::new(unsafe { IDsService::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IDsService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IDsService {
	pub fn bind_device(&self, complex_id: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(complex_id)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn bind_client_process(&self, unk0: &KObject) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			.copy_handle(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_ds_interface(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_state_change_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn get_state(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn set_vid_pid_bcd(&self, descriptor: &::nn::usb::UsbDescriptorData) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(5)
			.args(())
			.descriptor(IPCBuffer::from_ref(descriptor, 5))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IDsService {
	unsafe fn from_kobject(obj: KObject) -> IDsService {
		IDsService(Session::from_kobject(obj))
	}
}
