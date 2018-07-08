
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct INvDrvServices<T>(T);

impl INvDrvServices<Session> {
	pub fn raw_new_nvdrv_s(transfer_memory_size: u32, current_process: &KObject, transfer_memory: &KObject) -> Result<INvDrvServices<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"nvdrv:s\0")?;
		let object : Self = Session::from(session).into();
		let errcode = object.initialize(transfer_memory_size, current_process, transfer_memory)?;
		if errcode != 0 {
			return Err(Error(1))
		}
		Ok(object)
	}

	pub fn new_nvdrv_s<T: FnOnce(fn(u32, &KObject, &KObject) -> Result<INvDrvServices<Session>>) -> Result<INvDrvServices<Session>>>(f: T) -> Result<Arc<INvDrvServices<Session>>> {
		use alloc::arc::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<INvDrvServices<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"nvdrv:s\0") {
			let ret = Arc::new(INvDrvServices(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = f(Self::raw_new_nvdrv_s)?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn raw_new_nvdrv_t(transfer_memory_size: u32, current_process: &KObject, transfer_memory: &KObject) -> Result<INvDrvServices<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"nvdrv:t\0")?;
		let object : Self = Session::from(session).into();
		let errcode = object.initialize(transfer_memory_size, current_process, transfer_memory)?;
		if errcode != 0 {
			return Err(Error(1))
		}
		Ok(object)
	}

	pub fn new_nvdrv_t<T: FnOnce(fn(u32, &KObject, &KObject) -> Result<INvDrvServices<Session>>) -> Result<INvDrvServices<Session>>>(f: T) -> Result<Arc<INvDrvServices<Session>>> {
		use alloc::arc::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<INvDrvServices<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"nvdrv:t\0") {
			let ret = Arc::new(INvDrvServices(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = f(Self::raw_new_nvdrv_t)?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn raw_new_nvdrv_a(transfer_memory_size: u32, current_process: &KObject, transfer_memory: &KObject) -> Result<INvDrvServices<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"nvdrv:a\0")?;
		let object : Self = Session::from(session).into();
		let errcode = object.initialize(transfer_memory_size, current_process, transfer_memory)?;
		if errcode != 0 {
			return Err(Error(1))
		}
		Ok(object)
	}

	pub fn new_nvdrv_a<T: FnOnce(fn(u32, &KObject, &KObject) -> Result<INvDrvServices<Session>>) -> Result<INvDrvServices<Session>>>(f: T) -> Result<Arc<INvDrvServices<Session>>> {
		use alloc::arc::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<INvDrvServices<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"nvdrv:a\0") {
			let ret = Arc::new(INvDrvServices(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = f(Self::raw_new_nvdrv_a)?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn raw_new_nvdrv(transfer_memory_size: u32, current_process: &KObject, transfer_memory: &KObject) -> Result<INvDrvServices<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"nvdrv\0\0\0")?;
		let object : Self = Session::from(session).into();
		let errcode = object.initialize(transfer_memory_size, current_process, transfer_memory)?;
		if errcode != 0 {
			return Err(Error(1))
		}
		Ok(object)
	}

	pub fn new_nvdrv<T: FnOnce(fn(u32, &KObject, &KObject) -> Result<INvDrvServices<Session>>) -> Result<INvDrvServices<Session>>>(f: T) -> Result<Arc<INvDrvServices<Session>>> {
		use alloc::arc::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<INvDrvServices<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"nvdrv\0\0\0") {
			let ret = Arc::new(INvDrvServices(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = f(Self::raw_new_nvdrv)?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<INvDrvServices<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(INvDrvServices(domain)),
			Err((sess, err)) => Err((INvDrvServices(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<INvDrvServices<Session>> {
		Ok(INvDrvServices(self.0.duplicate()?))
	}
}

impl<T> Deref for INvDrvServices<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for INvDrvServices<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> INvDrvServices<T> {
	pub fn open(&self, path: &[i8]) -> Result<(u32, u32)> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(0)
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

	pub fn ioctl(&self, fd: u32, rq_id: u32, unk1: u64, inbuf: &[u8], outbuf: &mut [u8]) -> Result<u32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			fd: u32,
			rq_id: u32,
			unk1: u64,
		}
		let req : Request<_, [_; 2], [_; 0], [_; 0]> = Request::new(1)
			.args(InRaw {
				fd,
				rq_id,
				unk1,
			})
			.descriptor(IPCBuffer::from_slice(inbuf, 0x21))
			.descriptor(IPCBuffer::from_mut_slice(outbuf, 0x22))
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn close(&self, fd: u32) -> Result<u32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(fd)
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn initialize(&self, transfer_memory_size: u32, current_process: &KObject, transfer_memory: &KObject) -> Result<u32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 2], [_; 0]> = Request::new(3)
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

impl<T: Object> From<T> for INvDrvServices<T> {
	fn from(obj: T) -> INvDrvServices<T> {
		INvDrvServices(obj)
	}
}
