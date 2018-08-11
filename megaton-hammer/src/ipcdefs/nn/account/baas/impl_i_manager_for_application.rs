
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
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
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_account_id(&self, ) -> Result<::ipcdefs::nn::account::NetworkServiceAccountId> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let res : Response<::ipcdefs::nn::account::NetworkServiceAccountId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn ensure_id_token_cache_async(&self, ) -> Result<::ipcdefs::nn::account::detail::IAsyncContext<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	// fn load_id_token_cache(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_nintendo_account_user_resource_cache_for_application(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn create_authorization_request(&self, unk0: u32, unk1: &KObject, unk2: &::ipcdefs::nn::account::NintendoAccountAuthorizationRequestParameters) -> Result<::ipcdefs::nn::account::nas::IAuthorizationRequest<T>> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 1], [_; 0]> = Request::new(150)
			.args(unk0)
			.copy_handle(unk1)
			.descriptor(IPCBuffer::from_ref(unk2, 0x19))
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn store_open_context(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(160)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IManagerForApplication<T> {
	fn from(obj: T) -> IManagerForApplication<T> {
		IManagerForApplication(obj)
	}
}
