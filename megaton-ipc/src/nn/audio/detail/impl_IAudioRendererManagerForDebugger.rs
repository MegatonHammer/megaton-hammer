
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IAudioRendererManagerForDebugger(Session);

impl IAudioRendererManagerForDebugger {
	pub fn get_service() -> Result<IAudioRendererManagerForDebugger> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"audren:d").map(|s| unsafe { IAudioRendererManagerForDebugger::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl IAudioRendererManagerForDebugger {
	pub fn Unknown0(&self, unk0: u64) -> Result<()> {
		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown1(&self, unk0: u64) -> Result<()> {
		let req = Request::new(1)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IAudioRendererManagerForDebugger {
	unsafe fn from_kobject(obj: KObject) -> IAudioRendererManagerForDebugger {
		IAudioRendererManagerForDebugger(Session::from_kobject(obj))
	}
}
