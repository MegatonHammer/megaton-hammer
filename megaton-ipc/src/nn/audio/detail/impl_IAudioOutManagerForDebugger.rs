
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IAudioOutManagerForDebugger(Session);

impl IAudioOutManagerForDebugger {
	pub fn get_service() -> Result<IAudioOutManagerForDebugger> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"audout:d").map(|s| unsafe { IAudioOutManagerForDebugger::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IAudioOutManagerForDebugger {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
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
