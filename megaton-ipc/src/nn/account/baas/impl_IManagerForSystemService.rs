
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hammer::ipc::IPCBuffer;

#[derive(Debug)]
pub struct IManagerForSystemService(Session);

impl AsRef<Session> for IManagerForSystemService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IManagerForSystemService {
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
	// fn SetSystemProgramIdentification(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetNintendoAccountId(&self, ) -> Result<::nn::account::NintendoAccountId> {
		let req = Request::new(120)
			.args(())
			;
		let mut res : Response<::nn::account::NintendoAccountId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn GetNintendoAccountUserResourceCache(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn RefreshNintendoAccountUserResourceCacheAsync(&self, ) -> Result<::nn::account::detail::IAsyncContext> {
		let req = Request::new(131)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn RefreshNintendoAccountUserResourceCacheAsyncIfSecondsElapsed(&self, unk0: u32) -> Result<(bool, ::nn::account::detail::IAsyncContext)> {
		let req = Request::new(132)
			.args(unk0)
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok((*res.get_raw(),unsafe { FromKObject::from_kobject(res.pop_handle()) }))
	}

	// fn CreateAuthorizationRequest(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IManagerForSystemService {
	unsafe fn from_kobject(obj: KObject) -> IManagerForSystemService {
		IManagerForSystemService(Session::from_kobject(obj))
	}
}
