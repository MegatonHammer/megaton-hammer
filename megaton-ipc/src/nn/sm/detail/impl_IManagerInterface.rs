
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IManagerInterface(Session);

impl IManagerInterface {
	// fn RegisterProcess(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn UnregisterProcess(&self, unk0: u64) -> Result<()> {
		let req = Request::new(1)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
}

impl FromKObject for IManagerInterface {
	unsafe fn from_kobject(obj: KObject) -> IManagerInterface {
		IManagerInterface(Session::from_kobject(obj))
	}
}
