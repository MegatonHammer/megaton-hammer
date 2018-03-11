
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IApplicationCreator(Session);

impl AsRef<Session> for IApplicationCreator {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IApplicationCreator {
	pub fn create_application(&self, unk0: ::nn::ncm::ApplicationId) -> Result<::nn::am::service::IApplicationAccessor> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn pop_launch_requested_application(&self, ) -> Result<::nn::am::service::IApplicationAccessor> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn create_system_application(&self, unk0: ::nn::ncm::SystemApplicationId) -> Result<::nn::am::service::IApplicationAccessor> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn pop_floating_application_for_development(&self, ) -> Result<::nn::am::service::IApplicationAccessor> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(100)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IApplicationCreator {
	unsafe fn from_kobject(obj: KObject) -> IApplicationCreator {
		IApplicationCreator(Session::from_kobject(obj))
	}
}
