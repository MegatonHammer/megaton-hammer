
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IOAuthProcedureForNintendoAccountLinkage(Session);

impl IOAuthProcedureForNintendoAccountLinkage {
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
	pub fn GetRequestWithTheme(&self, unk0: i32, unk1: &mut Option<::nn::account::RequestUrl>, unk2: &mut Option<::nn::account::CallbackUri>) -> Result<()> {
		let req = Request::new(100)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn IsNetworkServiceAccountReplaced(&self, ) -> Result<bool> {
		let req = Request::new(101)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetUrlForIntroductionOfExtraMembership(&self, unk0: &mut Option<::nn::account::RequestUrl>) -> Result<()> {
		let req = Request::new(199)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
}

impl FromKObject for IOAuthProcedureForNintendoAccountLinkage {
	unsafe fn from_kobject(obj: KObject) -> IOAuthProcedureForNintendoAccountLinkage {
		IOAuthProcedureForNintendoAccountLinkage(Session::from_kobject(obj))
	}
}
