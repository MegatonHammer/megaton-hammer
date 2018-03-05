
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IAudioOutManagerForDebugger(Session);

impl IAudioOutManagerForDebugger {
	pub fn RequestSuspendForDebug(&self, unk0: u64) -> Result<()> {
		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn RequestResumeForDebug(&self, unk0: u64) -> Result<()> {
		let req = Request::new(1)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
}

impl FromKObject for IAudioOutManagerForDebugger {
	unsafe fn from_kobject(obj: KObject) -> IAudioOutManagerForDebugger {
		IAudioOutManagerForDebugger(Session::from_kobject(obj))
	}
}
