
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct INvDrvServices(Session);

impl INvDrvServices {
	pub fn new_nvdrv_s() -> Result<Arc<INvDrvServices>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<INvDrvServices>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"nvdrv:s\0") {
			let ret = Arc::new(INvDrvServices(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"nvdrv:s\0").map(|s| Arc::new(unsafe { INvDrvServices::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
	pub fn new_nvdrv_t() -> Result<Arc<INvDrvServices>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<INvDrvServices>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"nvdrv:t\0") {
			let ret = Arc::new(INvDrvServices(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"nvdrv:t\0").map(|s| Arc::new(unsafe { INvDrvServices::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
	pub fn new_nvdrv_a() -> Result<Arc<INvDrvServices>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<INvDrvServices>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"nvdrv:a\0") {
			let ret = Arc::new(INvDrvServices(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"nvdrv:a\0").map(|s| Arc::new(unsafe { INvDrvServices::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
	pub fn new_nvdrv() -> Result<Arc<INvDrvServices>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<INvDrvServices>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"nvdrv\0\0\0") {
			let ret = Arc::new(INvDrvServices(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"nvdrv\0\0\0").map(|s| Arc::new(unsafe { INvDrvServices::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for INvDrvServices {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl INvDrvServices {
	pub fn open(&self, path: &[i8]) -> Result<(u32, u32)> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			.descriptor(IPCBuffer::from_slice(path, 5))
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			fd: u32,
			error_code: u32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().fd.clone(),res.get_raw().error_code.clone()))
	}

	pub fn ioctl(&self, fd: u32, rq_id: u32, unk2: u32, unk3: u32) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			fd: u32,
			rq_id: u32,
			unk2: u32,
			unk3: u32,
		}
		let req = Request::new(1)
			.args(InRaw {
				fd,
				rq_id,
				unk2,
				unk3,
			})
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn close(&self, fd: u32) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(fd)
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn initialize(&self, transfer_memory_size: u32, current_process: &KObject, transfer_memory: &KObject) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(transfer_memory_size)
			.copy_handle(current_process)
			.copy_handle(transfer_memory)
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn query_event(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn map_shared_mem(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_status(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn force_set_client_pid(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_client_pid(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn dump_graphics_memory_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown10(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn ioctl2(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn ioctl3(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown13(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for INvDrvServices {
	unsafe fn from_kobject(obj: KObject) -> INvDrvServices {
		INvDrvServices(Session::from_kobject(obj))
	}
}
