
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IOAuthProcedureForNintendoAccountLinkage(Session);

impl AsRef<Session> for IOAuthProcedureForNintendoAccountLinkage {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IOAuthProcedureForNintendoAccountLinkage {
	pub fn prepare_async(&self, ) -> Result<::nn::account::detail::IAsyncContext> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
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

	pub fn apply_response_async(&self, unk0: &[i8]) -> Result<::nn::account::detail::IAsyncContext> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			.descriptor(IPCBuffer::from_slice(unk0, 9))
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn suspend(&self, ) -> Result<::nn::account::detail::Uuid> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(())
			;
		let res : Response<::nn::account::detail::Uuid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_request_with_theme(&self, unk0: i32, unk1: &mut ::nn::account::RequestUrl, unk2: &mut ::nn::account::CallbackUri) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(100)
			.args(unk0)
			.descriptor(IPCBuffer::from_mut_ref(unk1, 0x1a))
			.descriptor(IPCBuffer::from_mut_ref(unk2, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_network_service_account_replaced(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(101)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_url_for_introduction_of_extra_membership(&self, unk0: &mut ::nn::account::RequestUrl) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(199)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IOAuthProcedureForNintendoAccountLinkage {
	unsafe fn from_kobject(obj: KObject) -> IOAuthProcedureForNintendoAccountLinkage {
		IOAuthProcedureForNintendoAccountLinkage(Session::from_kobject(obj))
	}
}
