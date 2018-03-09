
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IManagerForApplication(Session);

impl AsRef<Session> for IManagerForApplication {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IManagerForApplication {
	pub fn CheckAvailability(&self, ) -> Result<()> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetAccountId(&self, ) -> Result<::nn::account::NetworkServiceAccountId> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<::nn::account::NetworkServiceAccountId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn EnsureIdTokenCacheAsync(&self, ) -> Result<::nn::account::detail::IAsyncContext> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	// fn LoadIdTokenCache(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetNintendoAccountUserResourceCacheForApplication(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn CreateAuthorizationRequest(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IManagerForApplication {
	unsafe fn from_kobject(obj: KObject) -> IManagerForApplication {
		IManagerForApplication(Session::from_kobject(obj))
	}
}
