
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IFinalOutputRecorderManagerForDebugger(Session);

impl IFinalOutputRecorderManagerForDebugger {
	pub fn get_service() -> Result<IFinalOutputRecorderManagerForDebugger> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"audrec:d").map(|s| unsafe { IFinalOutputRecorderManagerForDebugger::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IFinalOutputRecorderManagerForDebugger {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IFinalOutputRecorderManagerForDebugger {
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

impl FromKObject for IFinalOutputRecorderManagerForDebugger {
	unsafe fn from_kobject(obj: KObject) -> IFinalOutputRecorderManagerForDebugger {
		IFinalOutputRecorderManagerForDebugger(Session::from_kobject(obj))
	}
}
