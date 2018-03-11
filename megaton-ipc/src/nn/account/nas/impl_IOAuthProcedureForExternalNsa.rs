
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hamer::ipc::IPCBuffer

#[derive(Debug)]
pub struct IOAuthProcedureForExternalNsa(Session);

impl AsRef<Session> for IOAuthProcedureForExternalNsa {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IOAuthProcedureForExternalNsa {
	pub fn PrepareAsync(&self, ) -> Result<::nn::account::detail::IAsyncContext> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	// fn GetRequest(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn ApplyResponse(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn ApplyResponseAsync(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Suspend(&self, ) -> Result<::nn::account::detail::Uuid> {
		let req = Request::new(10)
			.args(())
			;
		let mut res : Response<::nn::account::detail::Uuid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetAccountId(&self, ) -> Result<::nn::account::NetworkServiceAccountId> {
		let req = Request::new(100)
			.args(())
			;
		let mut res : Response<::nn::account::NetworkServiceAccountId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetLinkedNintendoAccountId(&self, ) -> Result<::nn::account::NintendoAccountId> {
		let req = Request::new(101)
			.args(())
			;
		let mut res : Response<::nn::account::NintendoAccountId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn GetNickname(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetProfileImage(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IOAuthProcedureForExternalNsa {
	unsafe fn from_kobject(obj: KObject) -> IOAuthProcedureForExternalNsa {
		IOAuthProcedureForExternalNsa(Session::from_kobject(obj))
	}
}
