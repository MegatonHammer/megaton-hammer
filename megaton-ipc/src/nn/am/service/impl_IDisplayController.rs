
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IDisplayController(Session);

impl IDisplayController {
	// fn GetLastForegroundCaptureImage(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn UpdateLastForegroundCaptureImage(&self, ) -> Result<()> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn GetLastApplicationCaptureImage(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetCallerAppletCaptureImage(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn UpdateCallerAppletCaptureImage(&self, ) -> Result<()> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn GetLastForegroundCaptureImageEx(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetLastApplicationCaptureImageEx(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetCallerAppletCaptureImageEx(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn TakeScreenShotOfOwnLayer(&self, unk0: bool, unk1: i32) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: i32,
		}
		let req = Request::new(8)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn AcquireLastApplicationCaptureBuffer(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn ReleaseLastApplicationCaptureBuffer(&self, ) -> Result<()> {
		let req = Request::new(11)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn AcquireLastForegroundCaptureBuffer(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn ReleaseLastForegroundCaptureBuffer(&self, ) -> Result<()> {
		let req = Request::new(13)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn AcquireCallerAppletCaptureBuffer(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn ReleaseCallerAppletCaptureBuffer(&self, ) -> Result<()> {
		let req = Request::new(15)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn AcquireLastApplicationCaptureBufferEx(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn AcquireLastForegroundCaptureBufferEx(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn AcquireCallerAppletCaptureBufferEx(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn ClearCaptureBuffer(&self, unk0: bool, unk1: i32, unk2: u32) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: i32,
			unk2: u32,
		}
		let req = Request::new(20)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn ClearAppletTransitionBuffer(&self, unk0: u32) -> Result<()> {
		let req = Request::new(21)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
}

impl FromKObject for IDisplayController {
	unsafe fn from_kobject(obj: KObject) -> IDisplayController {
		IDisplayController(Session::from_kobject(obj))
	}
}
