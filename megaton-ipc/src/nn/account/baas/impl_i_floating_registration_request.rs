
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IFloatingRegistrationRequest(Session);

impl AsRef<Session> for IFloatingRegistrationRequest {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IFloatingRegistrationRequest {
	pub fn get_session_id(&self, ) -> Result<::nn::account::detail::Uuid> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let res : Response<::nn::account::detail::Uuid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_account_id(&self, ) -> Result<::nn::account::NetworkServiceAccountId> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(12)
			.args(())
			;
		let res : Response<::nn::account::NetworkServiceAccountId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_linked_nintendo_account_id(&self, ) -> Result<::nn::account::NintendoAccountId> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(13)
			.args(())
			;
		let res : Response<::nn::account::NintendoAccountId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_nickname(&self, unk0: &mut [i8]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(14)
			.args(())
			.descriptor(IPCBuffer::from_mut_slice(unk0, 0xa))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_profile_image(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn load_id_token_cache(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn register_async(&self, ) -> Result<(::nn::account::Uid, ::nn::account::detail::IAsyncContext)> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(100)
			.args(())
			;
		let mut res : Response<::nn::account::Uid> = self.0.send(req)?;
		Ok((*res.get_raw(),unsafe { FromKObject::from_kobject(res.pop_handle()) }))
	}

	pub fn register_with_uid_async(&self, unk0: ::nn::account::Uid) -> Result<::nn::account::detail::IAsyncContext> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(101)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn set_system_program_identification(&self, unk0: u64, unk2: &::nn::account::SystemProgramIdentification) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(110)
			.args(unk0)
			.send_pid()
			.descriptor(IPCBuffer::from_ref(unk2, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ensure_id_token_cache_async(&self, ) -> Result<::nn::account::detail::IAsyncContext> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(111)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IFloatingRegistrationRequest {
	unsafe fn from_kobject(obj: KObject) -> IFloatingRegistrationRequest {
		IFloatingRegistrationRequest(Session::from_kobject(obj))
	}
}
