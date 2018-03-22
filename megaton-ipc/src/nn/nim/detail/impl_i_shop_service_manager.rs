
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IShopServiceManager(Session);

impl IShopServiceManager {
	pub fn new() -> Result<Arc<IShopServiceManager>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IShopServiceManager>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"nim:shp\0") {
			let ret = Arc::new(IShopServiceManager(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"nim:shp\0").map(|s| Arc::new(unsafe { IShopServiceManager::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IShopServiceManager {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IShopServiceManager {
	// fn request_device_authentication_token(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn request_cached_device_authentication_token(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn request_register_device_account(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn request_unregister_device_account(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn request_device_account_status(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_device_account_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn request_device_registration_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn request_transfer_device_account(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn request_sync_registration(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn is_own_device_id(&self, unk0: u64) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(107)
			.args(unk0)
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn request_register_notification_token(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn request_unlink_device(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn request_unlink_device_integrated(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn request_link_device(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn has_device_link(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn request_unlink_device_all(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(304)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn request_create_virtual_account(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(305)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn request_device_link_status(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(306)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_account_by_virtual_account(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn request_sync_ticket(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn request_download_ticket(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn request_download_ticket_for_prepurchased_contents(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(502)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IShopServiceManager {
	unsafe fn from_kobject(obj: KObject) -> IShopServiceManager {
		IShopServiceManager(Session::from_kobject(obj))
	}
}
