
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IOAuthProcedureForExternalNsa(Session);

impl IOAuthProcedureForExternalNsa {
	pub fn PrepareAsync(&self, ) -> Result<::nn::account::detail::IAsyncContext> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn GetRequest(&self, unk0: &mut Option<::nn::account::RequestUrl>, unk1: &mut Option<::nn::account::CallbackUri>) -> Result<()> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ApplyResponse(&self, unk0: &[i8]) -> Result<()> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ApplyResponseAsync(&self, unk0: &[i8]) -> Result<::nn::account::detail::IAsyncContext> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

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

	pub fn GetNickname(&self, unk0: &mut [i8]) -> Result<()> {
		let req = Request::new(102)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn GetProfileImage(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IOAuthProcedureForExternalNsa {
	unsafe fn from_kobject(obj: KObject) -> IOAuthProcedureForExternalNsa {
		IOAuthProcedureForExternalNsa(Session::from_kobject(obj))
	}
}
