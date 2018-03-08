
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IManagerForApplication(Session);

impl IManagerForApplication {
	pub fn CheckAvailability(&self, ) -> Result<()> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetAccountId(&self, ) -> Result<(::nn::account::NetworkServiceAccountId)> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<::nn::account::NetworkServiceAccountId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn EnsureIdTokenCacheAsync(&self, ) -> Result<(::nn::account::detail::IAsyncContext)> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	// fn LoadIdTokenCache(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetNintendoAccountUserResourceCacheForApplication(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn CreateAuthorizationRequest(&self, unk0: u32, unk1: KObject, unk2: &::nn::account::NintendoAccountAuthorizationRequestParameters) -> Result<(::nn::account::nas::IAuthorizationRequest)> {
		let req = Request::new(150)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IManagerForApplication {
	unsafe fn from_kobject(obj: KObject) -> IManagerForApplication {
		IManagerForApplication(Session::from_kobject(obj))
	}
}
