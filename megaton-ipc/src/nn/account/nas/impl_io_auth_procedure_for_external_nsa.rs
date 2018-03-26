
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IOAuthProcedureForExternalNsa<T>(T);

impl IOAuthProcedureForExternalNsa<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IOAuthProcedureForExternalNsa<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IOAuthProcedureForExternalNsa(domain)),
			Err((sess, err)) => Err((IOAuthProcedureForExternalNsa(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IOAuthProcedureForExternalNsa<Session>> {
		Ok(IOAuthProcedureForExternalNsa(self.0.duplicate()?))
	}
}

impl<T> Deref for IOAuthProcedureForExternalNsa<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IOAuthProcedureForExternalNsa<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IOAuthProcedureForExternalNsa<T> {
	pub fn prepare_async(&self, ) -> Result<::nn::account::detail::IAsyncContext<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_request(&self, unk0: &mut ::nn::account::RequestUrl, unk1: &mut ::nn::account::CallbackUri) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x1a))
			.descriptor(IPCBuffer::from_mut_ref(unk1, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn apply_response(&self, unk0: &[i8]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			.descriptor(IPCBuffer::from_slice(unk0, 9))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn apply_response_async(&self, unk0: &[i8]) -> Result<::nn::account::detail::IAsyncContext<T>> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			.descriptor(IPCBuffer::from_slice(unk0, 9))
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn suspend(&self, ) -> Result<::nn::account::detail::Uuid> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(())
			;
		let res : Response<::nn::account::detail::Uuid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_account_id(&self, ) -> Result<::nn::account::NetworkServiceAccountId> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(100)
			.args(())
			;
		let res : Response<::nn::account::NetworkServiceAccountId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_linked_nintendo_account_id(&self, ) -> Result<::nn::account::NintendoAccountId> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(101)
			.args(())
			;
		let res : Response<::nn::account::NintendoAccountId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_nickname(&self, unk0: &mut [i8]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(102)
			.args(())
			.descriptor(IPCBuffer::from_mut_slice(unk0, 0xa))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_profile_image(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IOAuthProcedureForExternalNsa<T> {
	fn from(obj: T) -> IOAuthProcedureForExternalNsa<T> {
		IOAuthProcedureForExternalNsa(obj)
	}
}
