
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IOAuthProcedureForNintendoAccountLinkage(Session);

impl AsRef<Session> for IOAuthProcedureForNintendoAccountLinkage {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IOAuthProcedureForNintendoAccountLinkage {
	pub fn prepare_async(&self, ) -> Result<::nn::account::detail::IAsyncContext> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	// fn get_request(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn apply_response(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn apply_response_async(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn suspend(&self, ) -> Result<::nn::account::detail::Uuid> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(())
			;
		let res : Response<::nn::account::detail::Uuid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_request_with_theme(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn is_network_service_account_replaced(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(101)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_url_for_introduction_of_extra_membership(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IOAuthProcedureForNintendoAccountLinkage {
	unsafe fn from_kobject(obj: KObject) -> IOAuthProcedureForNintendoAccountLinkage {
		IOAuthProcedureForNintendoAccountLinkage(Session::from_kobject(obj))
	}
}
