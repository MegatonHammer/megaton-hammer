
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IDebugger(Session);

impl IDebugger {
	// fn Initialize(&self, UNKNOWN) -> Result<UNKNOWN>;
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
