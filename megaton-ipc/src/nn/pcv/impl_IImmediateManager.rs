
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IImmediateManager(Session);

impl IImmediateManager {
	pub fn SetClockRate(&self, unk0: i32, unk1: u32) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: u32,
		}
		let req = Request::new(0)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
}

impl FromKObject for IImmediateManager {
	unsafe fn from_kobject(obj: KObject) -> IImmediateManager {
		IImmediateManager(Session::from_kobject(obj))
	}
}
