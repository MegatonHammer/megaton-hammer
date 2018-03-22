
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IManager(Session);

impl IManager {
	pub fn new_nsd_a() -> Result<Arc<IManager>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IManager>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"nsd:a\0\0\0") {
			let ret = Arc::new(IManager(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"nsd:a\0\0\0").map(|s| Arc::new(unsafe { IManager::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
	pub fn new_nsd_u() -> Result<Arc<IManager>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IManager>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"nsd:u\0\0\0") {
			let ret = Arc::new(IManager(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"nsd:u\0\0\0").map(|s| Arc::new(unsafe { IManager::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IManager {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IManager {
	pub fn get_setting_name(&self, unk0: &mut [u8; 0x100]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x16))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_environment_identifier(&self, unk0: &mut [u8; 8]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(11)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x16))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_device_id(&self, ) -> Result<u128> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(12)
			.args(())
			;
		let res : Response<u128> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn delete_settings(&self, unk0: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(13)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn import_settings(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn resolve(&self, unk0: &[u8; 0x100], unk1: &mut [u8; 0x100]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(20)
			.args(())
			.descriptor(IPCBuffer::from_ref(unk0, 0x15))
			.descriptor(IPCBuffer::from_mut_ref(unk1, 0x16))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn resolve_ex(&self, unk0: &[u8; 0x100], unk2: &mut [u8; 0x100]) -> Result<u32> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(21)
			.args(())
			.descriptor(IPCBuffer::from_ref(unk0, 0x15))
			.descriptor(IPCBuffer::from_mut_ref(unk2, 0x16))
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_nas_service_setting(&self, unk0: &[u8; 0x10], unk1: &mut [u8; 0x108]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(30)
			.args(())
			.descriptor(IPCBuffer::from_ref(unk0, 0x15))
			.descriptor(IPCBuffer::from_mut_ref(unk1, 0x16))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_nas_service_setting_ex(&self, unk0: &[u8; 0x10], unk2: &mut [u8; 0x108]) -> Result<u32> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(31)
			.args(())
			.descriptor(IPCBuffer::from_ref(unk0, 0x15))
			.descriptor(IPCBuffer::from_mut_ref(unk2, 0x16))
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_nas_request_fqdn(&self, unk0: &mut [u8; 0x100]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(40)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x16))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_nas_request_fqdn_ex(&self, unk1: &mut [u8; 0x100]) -> Result<u32> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(41)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk1, 0x16))
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_nas_api_fqdn(&self, unk0: &mut [u8; 0x100]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(42)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x16))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_nas_api_fqdn_ex(&self, unk1: &mut [u8; 0x100]) -> Result<u32> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(43)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk1, 0x16))
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_current_setting(&self, unk0: &mut [u8; 0x12bf0]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(50)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x16))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn read_save_data_from_fs_for_test(&self, unk0: &mut [u8; 0x12bf0]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(60)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x16))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn write_save_data_to_fs_for_test(&self, unk0: &[u8; 0x12bf0]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(61)
			.args(())
			.descriptor(IPCBuffer::from_ref(unk0, 0x15))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn delete_save_data_of_fs_for_test(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(62)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IManager {
	unsafe fn from_kobject(obj: KObject) -> IManager {
		IManager(Session::from_kobject(obj))
	}
}
