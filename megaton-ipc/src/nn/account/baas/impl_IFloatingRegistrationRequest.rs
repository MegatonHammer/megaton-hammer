
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hammer::ipc::IPCBuffer;

#[derive(Debug)]
pub struct IFloatingRegistrationRequest(Session);

impl AsRef<Session> for IFloatingRegistrationRequest {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IFloatingRegistrationRequest {
	pub fn GetSessionId(&self, ) -> Result<::nn::account::detail::Uuid> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<::nn::account::detail::Uuid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetAccountId(&self, ) -> Result<::nn::account::NetworkServiceAccountId> {
		let req = Request::new(12)
			.args(())
			;
		let mut res : Response<::nn::account::NetworkServiceAccountId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetLinkedNintendoAccountId(&self, ) -> Result<::nn::account::NintendoAccountId> {
		let req = Request::new(13)
			.args(())
			;
		let mut res : Response<::nn::account::NintendoAccountId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn GetNickname(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetProfileImage(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn LoadIdTokenCache(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn RegisterAsync(&self, ) -> Result<(::nn::account::Uid, ::nn::account::detail::IAsyncContext)> {
		let req = Request::new(100)
			.args(())
			;
		let mut res : Response<::nn::account::Uid> = self.0.send(req)?;
		Ok((*res.get_raw(),unsafe { FromKObject::from_kobject(res.pop_handle()) }))
	}

	pub fn RegisterWithUidAsync(&self, unk0: ::nn::account::Uid) -> Result<::nn::account::detail::IAsyncContext> {
		let req = Request::new(101)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	// fn SetSystemProgramIdentification(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn EnsureIdTokenCacheAsync(&self, ) -> Result<::nn::account::detail::IAsyncContext> {
		let req = Request::new(111)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IFloatingRegistrationRequest {
	unsafe fn from_kobject(obj: KObject) -> IFloatingRegistrationRequest {
		IFloatingRegistrationRequest(Session::from_kobject(obj))
	}
}
