
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IManagerForApplication(Session);

impl AsRef<Session> for IManagerForApplication {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IManagerForApplication {
	pub fn check_availability(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_account_id(&self, ) -> Result<::nn::account::NetworkServiceAccountId> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let res : Response<::nn::account::NetworkServiceAccountId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn ensure_id_token_cache_async(&self, ) -> Result<::nn::account::detail::IAsyncContext> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	// fn load_id_token_cache(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_nintendo_account_user_resource_cache_for_application(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn create_authorization_request(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IManagerForApplication {
	unsafe fn from_kobject(obj: KObject) -> IManagerForApplication {
		IManagerForApplication(Session::from_kobject(obj))
	}
}
