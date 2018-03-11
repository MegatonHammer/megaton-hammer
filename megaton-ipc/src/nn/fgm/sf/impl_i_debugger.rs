
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IDebugger(Session);

impl AsRef<Session> for IDebugger {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IDebugger {
	pub fn initialize(&self, unk0: u64, unk1: &KObject) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(unk0)
			.copy_handle(unk1)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn read(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn cancel(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IDebugger {
	unsafe fn from_kobject(obj: KObject) -> IDebugger {
		IDebugger(Session::from_kobject(obj))
	}
}
