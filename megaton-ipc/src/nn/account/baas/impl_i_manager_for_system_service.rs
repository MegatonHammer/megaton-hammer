
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IManagerForSystemService<T>(T);

impl IManagerForSystemService<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IManagerForSystemService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IManagerForSystemService(domain)),
			Err((sess, err)) => Err((IManagerForSystemService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IManagerForSystemService<Session>> {
		Ok(IManagerForSystemService(self.0.duplicate()?))
	}
}

impl<T> Deref for IManagerForSystemService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IManagerForSystemService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IManagerForSystemService<T> {
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

	pub fn ensure_id_token_cache_async(&self, ) -> Result<::nn::account::detail::IAsyncContext<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	// fn load_id_token_cache(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn set_system_program_identification(&self, unk0: u64, unk2: &::nn::account::SystemProgramIdentification) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(100)
			.args(unk0)
			.send_pid()
			.descriptor(IPCBuffer::from_ref(unk2, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_nintendo_account_id(&self, ) -> Result<::nn::account::NintendoAccountId> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(120)
			.args(())
			;
		let res : Response<::nn::account::NintendoAccountId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_nintendo_account_user_resource_cache(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn refresh_nintendo_account_user_resource_cache_async(&self, ) -> Result<::nn::account::detail::IAsyncContext<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(131)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn refresh_nintendo_account_user_resource_cache_async_if_seconds_elapsed(&self, unk0: u32) -> Result<(bool, ::nn::account::detail::IAsyncContext<T>)> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(132)
			.args(unk0)
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok((*res.get_raw(),T::from_res(&mut res).into()))
	}

	pub fn create_authorization_request(&self, unk0: u32, unk1: &KObject, unk2: &::nn::account::nas::NasClientInfo, unk3: &::nn::account::NintendoAccountAuthorizationRequestParameters) -> Result<::nn::account::nas::IAuthorizationRequest<T>> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(150)
			.args(unk0)
			.copy_handle(unk1)
			.descriptor(IPCBuffer::from_ref(unk2, 0x19))
			.descriptor(IPCBuffer::from_ref(unk3, 0x19))
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

}

impl<T: Object> From<T> for IManagerForSystemService<T> {
	fn from(obj: T) -> IManagerForSystemService<T> {
		IManagerForSystemService(obj)
	}
}
