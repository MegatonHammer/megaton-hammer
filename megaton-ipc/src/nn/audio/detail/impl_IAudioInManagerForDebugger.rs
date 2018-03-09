
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IAudioInManagerForDebugger(Session);

impl IAudioInManagerForDebugger {
	pub fn get_service() -> Result<IAudioInManagerForDebugger> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"audin:d\0").map(|s| unsafe { IAudioInManagerForDebugger::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IAudioInManagerForDebugger {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
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
