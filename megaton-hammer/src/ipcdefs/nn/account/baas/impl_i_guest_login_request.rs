
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IGuestLoginRequest<T>(T);

impl IGuestLoginRequest<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IGuestLoginRequest<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IGuestLoginRequest(domain)),
			Err((sess, err)) => Err((IGuestLoginRequest(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IGuestLoginRequest<Session>> {
		Ok(IGuestLoginRequest(self.0.duplicate()?))
	}
}

impl<T> Deref for IGuestLoginRequest<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IGuestLoginRequest<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IGuestLoginRequest<T> {
	pub fn get_session_id(&self, ) -> Result<::ipcdefs::nn::account::detail::Uuid> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let res : Response<::ipcdefs::nn::account::detail::Uuid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_account_id(&self, ) -> Result<::ipcdefs::nn::account::NetworkServiceAccountId> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(12)
			.args(())
			;
		let res : Response<::ipcdefs::nn::account::NetworkServiceAccountId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_linked_nintendo_account_id(&self, ) -> Result<::ipcdefs::nn::account::NintendoAccountId> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(13)
			.args(())
			;
		let res : Response<::ipcdefs::nn::account::NintendoAccountId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_nickname(&self, unk0: &mut [i8]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(14)
			.args(())
			.descriptor(IPCBuffer::from_mut_slice(unk0, 0xa))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_profile_image(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn load_id_token_cache(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IGuestLoginRequest<T> {
	fn from(obj: T) -> IGuestLoginRequest<T> {
		IGuestLoginRequest(obj)
	}
}
