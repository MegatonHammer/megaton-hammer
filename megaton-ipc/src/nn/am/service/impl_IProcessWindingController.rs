
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IProcessWindingController(Session);

impl IProcessWindingController {
	pub fn GetLaunchReason(&self, ) -> Result<::nn::am::service::AppletProcessLaunchReason> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<::nn::am::service::AppletProcessLaunchReason> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn OpenCallingLibraryApplet(&self, ) -> Result<::nn::am::service::ILibraryAppletAccessor> {
		let req = Request::new(11)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn PushContext(&self, unk0: ::nn::am::service::IStorage) -> Result<()> {
		let req = Request::new(21)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn PopContext(&self, ) -> Result<::nn::am::service::IStorage> {
		let req = Request::new(22)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn CancelWindingReservation(&self, ) -> Result<()> {
		let req = Request::new(23)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn WindAndDoReserved(&self, ) -> Result<()> {
		let req = Request::new(30)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ReserveToStartAndWaitAndUnwindThis(&self, unk0: ::nn::am::service::ILibraryAppletAccessor) -> Result<()> {
		let req = Request::new(40)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IProcessWindingController {
	unsafe fn from_kobject(obj: KObject) -> IProcessWindingController {
		IProcessWindingController(Session::from_kobject(obj))
	}
}
