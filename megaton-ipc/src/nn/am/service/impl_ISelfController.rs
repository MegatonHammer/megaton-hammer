
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct ISelfController(Session);

impl ISelfController {
	pub fn Exit(&self, ) -> Result<()> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn LockExit(&self, ) -> Result<()> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn UnlockExit(&self, ) -> Result<()> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn EnterFatalSection(&self, ) -> Result<()> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn LeaveFatalSection(&self, ) -> Result<()> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetLibraryAppletLaunchableEvent(&self, ) -> Result<(KObject)> {
		let req = Request::new(9)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn SetScreenShotPermission(&self, unk0: i32) -> Result<()> {
		let req = Request::new(10)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetOperationModeChangedNotification(&self, unk0: bool) -> Result<()> {
		let req = Request::new(11)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetPerformanceModeChangedNotification(&self, unk0: bool) -> Result<()> {
		let req = Request::new(12)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetFocusHandlingMode(&self, unk0: bool, unk1: bool, unk2: bool) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: bool,
			unk2: bool,
		}
		let req = Request::new(13)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetRestartMessageEnabled(&self, unk0: bool) -> Result<()> {
		let req = Request::new(14)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetScreenShotAppletIdentityInfo(&self, unk0: ::nn::am::service::AppletIdentityInfo) -> Result<()> {
		let req = Request::new(15)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetOutOfFocusSuspendingEnabled(&self, unk0: bool) -> Result<()> {
		let req = Request::new(16)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetControllerFirmwareUpdateSection(&self, unk0: bool) -> Result<()> {
		let req = Request::new(17)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetRequiresCaptureButtonShortPressedMessage(&self, unk0: bool) -> Result<()> {
		let req = Request::new(18)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetScreenShotImageOrientation(&self, unk0: i32) -> Result<()> {
		let req = Request::new(19)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn CreateManagedDisplayLayer(&self, ) -> Result<(u64)> {
		let req = Request::new(40)
			.args(())
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SetHandlesRequestToDisplay(&self, unk0: bool) -> Result<()> {
		let req = Request::new(50)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ApproveToDisplay(&self, ) -> Result<()> {
		let req = Request::new(51)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn OverrideAutoSleepTimeAndDimmingTime(&self, unk0: i32, unk1: i32, unk2: i32, unk3: i32) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: i32,
			unk2: i32,
			unk3: i32,
		}
		let req = Request::new(60)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
				unk3,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetMediaPlaybackState(&self, unk0: bool) -> Result<()> {
		let req = Request::new(61)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetIdleTimeDetectionExtension(&self, unk0: u32) -> Result<()> {
		let req = Request::new(62)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetIdleTimeDetectionExtension(&self, ) -> Result<(u32)> {
		let req = Request::new(63)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SetInputDetectionSourceSet(&self, unk0: u32) -> Result<()> {
		let req = Request::new(64)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ReportUserIsActive(&self, ) -> Result<()> {
		let req = Request::new(65)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetCurrentIlluminance(&self, ) -> Result<(f32)> {
		let req = Request::new(66)
			.args(())
			;
		let mut res : Response<f32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn IsIlluminanceAvailable(&self, ) -> Result<(bool)> {
		let req = Request::new(67)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for ISelfController {
	unsafe fn from_kobject(obj: KObject) -> ISelfController {
		ISelfController(Session::from_kobject(obj))
	}
}
