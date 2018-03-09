
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct ISystemAppletProxy(Session);

impl AsRef<Session> for ISystemAppletProxy {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ISystemAppletProxy {
	pub fn GetCommonStateGetter(&self, ) -> Result<::nn::am::service::ICommonStateGetter> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn GetSelfController(&self, ) -> Result<::nn::am::service::ISelfController> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn GetWindowController(&self, ) -> Result<::nn::am::service::IWindowController> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn GetAudioController(&self, ) -> Result<::nn::am::service::IAudioController> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn GetDisplayController(&self, ) -> Result<::nn::am::service::IDisplayController> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn GetProcessWindingController(&self, ) -> Result<::nn::am::service::IProcessWindingController> {
		let req = Request::new(10)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn GetLibraryAppletCreator(&self, ) -> Result<::nn::am::service::ILibraryAppletCreator> {
		let req = Request::new(11)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn GetHomeMenuFunctions(&self, ) -> Result<::nn::am::service::IHomeMenuFunctions> {
		let req = Request::new(20)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn GetGlobalStateController(&self, ) -> Result<::nn::am::service::IGlobalStateController> {
		let req = Request::new(21)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn GetApplicationCreator(&self, ) -> Result<::nn::am::service::IApplicationCreator> {
		let req = Request::new(22)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn GetDebugFunctions(&self, ) -> Result<::nn::am::service::IDebugFunctions> {
		let req = Request::new(1000)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for ISystemAppletProxy {
	unsafe fn from_kobject(obj: KObject) -> ISystemAppletProxy {
		ISystemAppletProxy(Session::from_kobject(obj))
	}
}
