
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IDebugger(Session);

impl IDebugger {
	pub fn Initialize(&self, unk0: u64, unk1: KObject) -> Result<KObject> {
		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn Read(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Cancel(&self, ) -> Result<()> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IDebugger {
	unsafe fn from_kobject(obj: KObject) -> IDebugger {
		IDebugger(Session::from_kobject(obj))
	}
}
