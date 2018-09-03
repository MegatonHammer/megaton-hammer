
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::sync::Arc;

#[derive(Debug)]
pub struct IShopServiceManager<T>(T);

impl IShopServiceManager<Session> {
	pub fn raw_new() -> Result<IShopServiceManager<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"nim:shp\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IShopServiceManager<Session>>> {
		use alloc::sync::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IShopServiceManager<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"nim:shp\0") {
			let ret = Arc::new(IShopServiceManager(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IShopServiceManager<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IShopServiceManager(domain)),
			Err((sess, err)) => Err((IShopServiceManager(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IShopServiceManager<Session>> {
		Ok(IShopServiceManager(self.0.duplicate()?))
	}
}

impl<T> Deref for IShopServiceManager<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IShopServiceManager<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IShopServiceManager<T> {
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
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(107)
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
	// fn request_unlink_device_all(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn request_create_virtual_account(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn request_device_link_status(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_account_by_virtual_account(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn request_sync_ticket(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn request_download_ticket(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn request_download_ticket_for_prepurchased_contents(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IShopServiceManager<T> {
	fn from(obj: T) -> IShopServiceManager<T> {
		IShopServiceManager(obj)
	}
}
