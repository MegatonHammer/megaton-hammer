
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IOverlayFunctions(Session);

impl AsRef<Session> for IOverlayFunctions {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IOverlayFunctions {
	pub fn BeginToWatchShortHomeButtonMessage(&self, ) -> Result<()> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn EndToWatchShortHomeButtonMessage(&self, ) -> Result<()> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetApplicationIdForLogo(&self, ) -> Result<::nn::ncm::ApplicationId> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<::nn::ncm::ApplicationId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SetGpuTimeSliceBoost(&self, unk0: u64) -> Result<()> {
		let req = Request::new(3)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetAutoSleepTimeAndDimmingTimeEnabled(&self, unk0: bool) -> Result<()> {
		let req = Request::new(4)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn TerminateApplicationAndSetReason(&self, unk0: u32) -> Result<()> {
		let req = Request::new(5)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetScreenShotPermissionGlobally(&self, unk0: bool) -> Result<()> {
		let req = Request::new(6)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IOverlayFunctions {
	unsafe fn from_kobject(obj: KObject) -> IOverlayFunctions {
		IOverlayFunctions(Session::from_kobject(obj))
	}
}
