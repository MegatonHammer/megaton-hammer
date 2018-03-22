
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IFirmwareDebugSettingsServer(Session);

impl IFirmwareDebugSettingsServer {
	pub fn new() -> Result<Arc<IFirmwareDebugSettingsServer>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IFirmwareDebugSettingsServer>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"set:fd\0\0") {
			let ret = Arc::new(IFirmwareDebugSettingsServer(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"set:fd\0\0").map(|s| Arc::new(unsafe { IFirmwareDebugSettingsServer::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IFirmwareDebugSettingsServer {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IFirmwareDebugSettingsServer {
	// fn set_settings_item_value(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn reset_settings_item_value(&self, unk0: &::nn::settings::SettingsName, unk1: &::nn::settings::SettingsItemKey) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			.descriptor(IPCBuffer::from_ref(unk0, 0x19))
			.descriptor(IPCBuffer::from_ref(unk1, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn create_settings_item_key_iterator(&self, unk0: &::nn::settings::SettingsName) -> Result<::nn::settings::ISettingsItemKeyIterator> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(())
			.descriptor(IPCBuffer::from_ref(unk0, 0x19))
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn read_settings(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn reset_settings(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(11)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn set_web_inspector_flag(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(20)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn set_allowed_ssl_hosts(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(21)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn set_host_fs_mount_point(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(22)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IFirmwareDebugSettingsServer {
	unsafe fn from_kobject(obj: KObject) -> IFirmwareDebugSettingsServer {
		IFirmwareDebugSettingsServer(Session::from_kobject(obj))
	}
}
