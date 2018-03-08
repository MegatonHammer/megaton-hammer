
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IAudioInManagerForDebugger(Session);

impl IAudioInManagerForDebugger {
	pub fn ListAudioIns(&self, unk0: u64) -> Result<()> {
		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn OpenAudioIn(&self, unk0: u64) -> Result<()> {
		let req = Request::new(1)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IAudioInManagerForDebugger {
	unsafe fn from_kobject(obj: KObject) -> IAudioInManagerForDebugger {
		IAudioInManagerForDebugger(Session::from_kobject(obj))
	}
}
