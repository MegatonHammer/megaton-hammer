
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IOAuthProcedureForNintendoAccountLinkage(Session);

impl AsRef<Session> for IOAuthProcedureForNintendoAccountLinkage {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IOAuthProcedureForNintendoAccountLinkage {
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

	// fn GetRequestWithTheme(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn IsNetworkServiceAccountReplaced(&self, ) -> Result<bool> {
		let req = Request::new(101)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn GetUrlForIntroductionOfExtraMembership(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IOAuthProcedureForNintendoAccountLinkage {
	unsafe fn from_kobject(obj: KObject) -> IOAuthProcedureForNintendoAccountLinkage {
		IOAuthProcedureForNintendoAccountLinkage(Session::from_kobject(obj))
	}
}
