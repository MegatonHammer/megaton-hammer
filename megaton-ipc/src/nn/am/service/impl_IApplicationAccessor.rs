
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IApplicationAccessor(Session);

impl IApplicationAccessor {
	pub fn GetAppletStateChangedEvent(&self, ) -> Result<(KObject)> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn IsCompleted(&self, ) -> Result<(bool)> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn Start(&self, ) -> Result<()> {
		let req = Request::new(10)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn RequestExit(&self, ) -> Result<()> {
		let req = Request::new(20)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Terminate(&self, ) -> Result<()> {
		let req = Request::new(25)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetResult(&self, ) -> Result<()> {
		let req = Request::new(30)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn RequestForApplicationToGetForeground(&self, ) -> Result<()> {
		let req = Request::new(101)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn TerminateAllLibraryApplets(&self, ) -> Result<()> {
		let req = Request::new(110)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn AreAnyLibraryAppletsLeft(&self, ) -> Result<(bool)> {
		let req = Request::new(111)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetCurrentLibraryApplet(&self, ) -> Result<(::nn::am::service::IAppletAccessor)> {
		let req = Request::new(112)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn GetApplicationId(&self, ) -> Result<(::nn::ncm::ApplicationId)> {
		let req = Request::new(120)
			.args(())
			;
		let mut res : Response<::nn::ncm::ApplicationId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn PushLaunchParameter(&self, unk0: u32, unk1: ::nn::am::service::IStorage) -> Result<()> {
		let req = Request::new(121)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn GetApplicationControlProperty(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetApplicationLaunchProperty(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IApplicationAccessor {
	unsafe fn from_kobject(obj: KObject) -> IApplicationAccessor {
		IApplicationAccessor(Session::from_kobject(obj))
	}
}
