
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IAuthorizationRequest<T>(T);

impl IAuthorizationRequest<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IAuthorizationRequest<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IAuthorizationRequest(domain)),
			Err((sess, err)) => Err((IAuthorizationRequest(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IAuthorizationRequest<Session>> {
		Ok(IAuthorizationRequest(self.0.duplicate()?))
	}
}

impl<T> Deref for IAuthorizationRequest<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IAuthorizationRequest<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IAuthorizationRequest<T> {
	pub fn get_session_id(&self, ) -> Result<::nn::account::detail::Uuid> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let res : Response<::nn::account::detail::Uuid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn invoke_without_interaction_async(&self, ) -> Result<::nn::account::detail::IAsyncContext<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
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

impl<T: Object> From<T> for IAuthorizationRequest<T> {
	fn from(obj: T) -> IAuthorizationRequest<T> {
		IAuthorizationRequest(obj)
	}
}
