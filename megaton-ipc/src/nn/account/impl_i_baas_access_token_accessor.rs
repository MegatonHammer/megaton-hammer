
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IBaasAccessTokenAccessor(Session);

impl IBaasAccessTokenAccessor {
	pub fn new() -> Result<IBaasAccessTokenAccessor> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"acc:aa\0\0").map(|s| unsafe { IBaasAccessTokenAccessor::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IBaasAccessTokenAccessor {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IBaasAccessTokenAccessor {
	pub fn ensure_cache_async(&self, unk0: ::nn::account::Uid) -> Result<::nn::account::detail::IAsyncContext> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	// fn load_cache(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_device_account_id(&self, unk0: ::nn::account::Uid) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(unk0)
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn register_notification_token_async(&self, unk0: ::nn::npns::NotificationToken, unk1: ::nn::account::Uid) -> Result<::nn::account::detail::IAsyncContext> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::npns::NotificationToken,
			unk1: ::nn::account::Uid,
		}
		let req = Request::new(50)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn unregister_notification_token_async(&self, unk0: ::nn::account::Uid) -> Result<::nn::account::detail::IAsyncContext> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(51)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IBaasAccessTokenAccessor {
	unsafe fn from_kobject(obj: KObject) -> IBaasAccessTokenAccessor {
		IBaasAccessTokenAccessor(Session::from_kobject(obj))
	}
}
