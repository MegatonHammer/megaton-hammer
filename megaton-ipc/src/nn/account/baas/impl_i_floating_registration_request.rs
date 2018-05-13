
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IFloatingRegistrationRequest<T>(T);

impl IFloatingRegistrationRequest<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IFloatingRegistrationRequest<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IFloatingRegistrationRequest(domain)),
			Err((sess, err)) => Err((IFloatingRegistrationRequest(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IFloatingRegistrationRequest<Session>> {
		Ok(IFloatingRegistrationRequest(self.0.duplicate()?))
	}
}

impl<T> Deref for IFloatingRegistrationRequest<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IFloatingRegistrationRequest<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IFloatingRegistrationRequest<T> {
	pub fn get_session_id(&self, ) -> Result<::nn::account::detail::Uuid> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let res : Response<::nn::account::detail::Uuid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_account_id(&self, ) -> Result<::nn::account::NetworkServiceAccountId> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(12)
			.args(())
			;
		let res : Response<::nn::account::NetworkServiceAccountId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_linked_nintendo_account_id(&self, ) -> Result<::nn::account::NintendoAccountId> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(13)
			.args(())
			;
		let res : Response<::nn::account::NintendoAccountId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_nickname(&self, unk0: &mut [i8]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(14)
			.args(())
			.descriptor(IPCBuffer::from_mut_slice(unk0, 0xa))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_profile_image(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn load_id_token_cache(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn register_async(&self, ) -> Result<(::nn::account::Uid, ::nn::account::detail::IAsyncContext<T>)> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(100)
			.args(())
			;
		let mut res : Response<::nn::account::Uid> = self.0.send(req)?;
		Ok((*res.get_raw(),T::from_res(&mut res).into()))
	}

	pub fn register_with_uid_async(&self, unk0: ::nn::account::Uid) -> Result<::nn::account::detail::IAsyncContext<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(101)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn set_system_program_identification(&self, unk0: u64, unk2: &::nn::account::SystemProgramIdentification) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(110)
			.args(unk0)
			.send_pid()
			.descriptor(IPCBuffer::from_ref(unk2, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ensure_id_token_cache_async(&self, ) -> Result<::nn::account::detail::IAsyncContext<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(111)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

}

impl<T: Object> From<T> for IFloatingRegistrationRequest<T> {
	fn from(obj: T) -> IFloatingRegistrationRequest<T> {
		IFloatingRegistrationRequest(obj)
	}
}
