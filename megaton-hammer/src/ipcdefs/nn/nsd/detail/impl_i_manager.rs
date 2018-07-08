
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IManager<T>(T);

impl IManager<Session> {
	pub fn raw_new_nsd_a() -> Result<IManager<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"nsd:a\0\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new_nsd_a() -> Result<Arc<IManager<Session>>> {
		use alloc::arc::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IManager<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"nsd:a\0\0\0") {
			let ret = Arc::new(IManager(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new_nsd_a()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn raw_new_nsd_u() -> Result<IManager<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"nsd:u\0\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new_nsd_u() -> Result<Arc<IManager<Session>>> {
		use alloc::arc::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IManager<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"nsd:u\0\0\0") {
			let ret = Arc::new(IManager(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new_nsd_u()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IManager<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IManager(domain)),
			Err((sess, err)) => Err((IManager(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IManager<Session>> {
		Ok(IManager(self.0.duplicate()?))
	}
}

impl<T> Deref for IManager<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IManager<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IManager<T> {
	pub fn get_setting_name(&self, unk0: &mut [u8; 0x100]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(10)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x16))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_environment_identifier(&self, unk0: &mut [u8; 8]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(11)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x16))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_device_id(&self, ) -> Result<u128> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(12)
			.args(())
			;
		let res : Response<u128> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn delete_settings(&self, unk0: u32) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(13)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn import_settings(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn resolve(&self, unk0: &[u8; 0x100], unk1: &mut [u8; 0x100]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 2], [_; 0], [_; 0]> = Request::new(20)
			.args(())
			.descriptor(IPCBuffer::from_ref(unk0, 0x15))
			.descriptor(IPCBuffer::from_mut_ref(unk1, 0x16))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn resolve_ex(&self, unk0: &[u8; 0x100], unk2: &mut [u8; 0x100]) -> Result<u32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 2], [_; 0], [_; 0]> = Request::new(21)
			.args(())
			.descriptor(IPCBuffer::from_ref(unk0, 0x15))
			.descriptor(IPCBuffer::from_mut_ref(unk2, 0x16))
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_nas_service_setting(&self, unk0: &[u8; 0x10], unk1: &mut [u8; 0x108]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 2], [_; 0], [_; 0]> = Request::new(30)
			.args(())
			.descriptor(IPCBuffer::from_ref(unk0, 0x15))
			.descriptor(IPCBuffer::from_mut_ref(unk1, 0x16))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_nas_service_setting_ex(&self, unk0: &[u8; 0x10], unk2: &mut [u8; 0x108]) -> Result<u32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 2], [_; 0], [_; 0]> = Request::new(31)
			.args(())
			.descriptor(IPCBuffer::from_ref(unk0, 0x15))
			.descriptor(IPCBuffer::from_mut_ref(unk2, 0x16))
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_nas_request_fqdn(&self, unk0: &mut [u8; 0x100]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(40)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x16))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_nas_request_fqdn_ex(&self, unk1: &mut [u8; 0x100]) -> Result<u32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(41)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk1, 0x16))
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_nas_api_fqdn(&self, unk0: &mut [u8; 0x100]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(42)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x16))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_nas_api_fqdn_ex(&self, unk1: &mut [u8; 0x100]) -> Result<u32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(43)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk1, 0x16))
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_current_setting(&self, unk0: &mut [u8; 0x12bf0]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(50)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x16))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn read_save_data_from_fs_for_test(&self, unk0: &mut [u8; 0x12bf0]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(60)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x16))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn write_save_data_to_fs_for_test(&self, unk0: &[u8; 0x12bf0]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(61)
			.args(())
			.descriptor(IPCBuffer::from_ref(unk0, 0x15))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn delete_save_data_of_fs_for_test(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(62)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IManager<T> {
	fn from(obj: T) -> IManager<T> {
		IManager(obj)
	}
}
