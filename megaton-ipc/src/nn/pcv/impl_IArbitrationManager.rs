
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IArbitrationManager(Session);

impl IArbitrationManager {
	pub fn ReleaseControl(&self, unk0: i32) -> Result<()> {
		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
}

impl FromKObject for IArbitrationManager {
	unsafe fn from_kobject(obj: KObject) -> IArbitrationManager {
		IArbitrationManager(Session::from_kobject(obj))
	}
}
