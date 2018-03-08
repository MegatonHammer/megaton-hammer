
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IApplicationCreator(Session);

impl IApplicationCreator {
	pub fn CreateApplication(&self, unk0: ::nn::ncm::ApplicationId) -> Result<::nn::am::service::IApplicationAccessor> {
		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn PopLaunchRequestedApplication(&self, ) -> Result<::nn::am::service::IApplicationAccessor> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn CreateSystemApplication(&self, unk0: ::nn::ncm::SystemApplicationId) -> Result<::nn::am::service::IApplicationAccessor> {
		let req = Request::new(10)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn PopFloatingApplicationForDevelopment(&self, ) -> Result<::nn::am::service::IApplicationAccessor> {
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
