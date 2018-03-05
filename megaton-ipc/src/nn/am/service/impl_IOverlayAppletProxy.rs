
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IOverlayAppletProxy(Session);

impl IOverlayAppletProxy {
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
	pub fn GetOverlayFunctions(&self, ) -> Result<::nn::am::service::IOverlayFunctions> {
		let req = Request::new(20)
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

impl FromKObject for IOverlayAppletProxy {
	unsafe fn from_kobject(obj: KObject) -> IOverlayAppletProxy {
		IOverlayAppletProxy(Session::from_kobject(obj))
	}
}
