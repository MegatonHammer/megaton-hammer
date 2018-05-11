
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IManagerForApplication<T>(T);

impl IManagerForApplication<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IManagerForApplication<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IManagerForApplication(domain)),
			Err((sess, err)) => Err((IManagerForApplication(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IManagerForApplication<Session>> {
		Ok(IManagerForApplication(self.0.duplicate()?))
	}
}

impl<T> Deref for IManagerForApplication<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IManagerForApplication<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IManagerForApplication<T> {
	pub fn check_availability(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_account_id(&self, ) -> Result<::nn::account::NetworkServiceAccountId> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let res : Response<::nn::account::NetworkServiceAccountId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn ensure_id_token_cache_async(&self, ) -> Result<::nn::account::detail::IAsyncContext<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	// fn load_id_token_cache(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_nintendo_account_user_resource_cache_for_application(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn create_authorization_request(&self, unk0: u32, unk1: &KObject, unk2: &::nn::account::NintendoAccountAuthorizationRequestParameters) -> Result<::nn::account::nas::IAuthorizationRequest<T>> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 1], [_; 0]> = Request::new(150)
			.args(unk0)
			.copy_handle(unk1)
			.descriptor(IPCBuffer::from_ref(unk2, 0x19))
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

}

impl<T: Object> From<T> for IManagerForApplication<T> {
	fn from(obj: T) -> IManagerForApplication<T> {
		IManagerForApplication(obj)
	}
}
