
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hammer::ipc::IPCBuffer;

#[derive(Debug)]
pub struct IAuthorizationRequest(Session);

impl AsRef<Session> for IAuthorizationRequest {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IAuthorizationRequest {
	pub fn GetSessionId(&self, ) -> Result<::nn::account::detail::Uuid> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<::nn::account::detail::Uuid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn InvokeWithoutInteractionAsync(&self, ) -> Result<::nn::account::detail::IAsyncContext> {
		let req = Request::new(10)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn IsAuthorized(&self, ) -> Result<bool> {
		let req = Request::new(19)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn GetAuthorizationCode(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetIdToken(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetState(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IAuthorizationRequest {
	unsafe fn from_kobject(obj: KObject) -> IAuthorizationRequest {
		IAuthorizationRequest(Session::from_kobject(obj))
	}
}
