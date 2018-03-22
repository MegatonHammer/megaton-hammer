
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IAuthorizationRequest(Session);

impl AsRef<Session> for IAuthorizationRequest {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IAuthorizationRequest {
	pub fn get_session_id(&self, ) -> Result<::nn::account::detail::Uuid> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let res : Response<::nn::account::detail::Uuid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn invoke_without_interaction_async(&self, ) -> Result<::nn::account::detail::IAsyncContext> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn is_authorized(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(19)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_authorization_code(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_id_token(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_state(&self, unk0: &mut ::nn::account::nas::State) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(22)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IAuthorizationRequest {
	unsafe fn from_kobject(obj: KObject) -> IAuthorizationRequest {
		IAuthorizationRequest(Session::from_kobject(obj))
	}
}
