
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IBaasAccessTokenAccessor(Session);

impl IBaasAccessTokenAccessor {
	pub fn get_service() -> Result<IBaasAccessTokenAccessor> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"acc:aa\0\0").map(|s| unsafe { IBaasAccessTokenAccessor::from_kobject(s) });
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
	pub fn EnsureCacheAsync(&self, unk0: ::nn::account::Uid) -> Result<::nn::account::detail::IAsyncContext> {
		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	// fn LoadCache(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetDeviceAccountId(&self, unk0: ::nn::account::Uid) -> Result<u64> {
		let req = Request::new(2)
			.args(unk0)
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn RegisterNotificationTokenAsync(&self, unk0: ::nn::npns::NotificationToken, unk1: ::nn::account::Uid) -> Result<::nn::account::detail::IAsyncContext> {
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

	pub fn UnregisterNotificationTokenAsync(&self, unk0: ::nn::account::Uid) -> Result<::nn::account::detail::IAsyncContext> {
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
