
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct ILibraryAppletSelfAccessor(Session);

impl AsRef<Session> for ILibraryAppletSelfAccessor {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ILibraryAppletSelfAccessor {
	pub fn pop_in_data(&self, ) -> Result<::nn::am::service::IStorage> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn push_out_data(&self, unk0: &::nn::am::service::IStorage) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			.copy_handle(unk0.as_ref().as_ref())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn pop_interactive_in_data(&self, ) -> Result<::nn::am::service::IStorage> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn push_interactive_out_data(&self, unk0: &::nn::am::service::IStorage) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			.copy_handle(unk0.as_ref().as_ref())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_pop_in_data_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(5)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn get_pop_interactive_in_data_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(6)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn exit_process_and_return(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_library_applet_info(&self, ) -> Result<::nn::am::service::LibraryAppletInfo> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(11)
			.args(())
			;
		let res : Response<::nn::am::service::LibraryAppletInfo> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_main_applet_identity_info(&self, ) -> Result<::nn::am::service::AppletIdentityInfo> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(12)
			.args(())
			;
		let res : Response<::nn::am::service::AppletIdentityInfo> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn can_use_application_core(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(13)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_caller_applet_identity_info(&self, ) -> Result<::nn::am::service::AppletIdentityInfo> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(14)
			.args(())
			;
		let res : Response<::nn::am::service::AppletIdentityInfo> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_main_applet_application_control_property(&self, unk0: &mut ::nn::ns::ApplicationControlProperty) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(15)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x16))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_main_applet_storage_id(&self, ) -> Result<::nn::ncm::StorageId> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(16)
			.args(())
			;
		let res : Response<::nn::ncm::StorageId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_caller_applet_identity_info_stack(&self, unk1: &mut [::nn::am::service::AppletIdentityInfo]) -> Result<i32> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(17)
			.args(())
			.descriptor(IPCBuffer::from_mut_slice(unk1, 6))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn pop_extra_storage(&self, ) -> Result<::nn::am::service::IStorage> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(20)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn get_pop_extra_storage_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(25)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn unpop_in_data(&self, unk0: &::nn::am::service::IStorage) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(30)
			.args(())
			.copy_handle(unk0.as_ref().as_ref())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unpop_extra_storage(&self, unk0: &::nn::am::service::IStorage) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(31)
			.args(())
			.copy_handle(unk0.as_ref().as_ref())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_indirect_layer_producer_handle(&self, ) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(40)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn report_visible_error(&self, unk0: ::nn::err::ErrorCode) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(50)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for ILibraryAppletSelfAccessor {
	unsafe fn from_kobject(obj: KObject) -> ILibraryAppletSelfAccessor {
		ILibraryAppletSelfAccessor(Session::from_kobject(obj))
	}
}
