
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IFinalOutputRecorderManager(Session);

impl IFinalOutputRecorderManager {
	pub fn OpenFinalOutputRecorder(&self, unk0: u64, unk1: u64, unk2: KObject) -> Result<(u128, Session)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u64,
		}
		let req = Request::new(0)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<u128> = self.0.send(req)?;
		Ok((*res.get_raw(),unsafe { FromKObject::from_kobject(res.pop_handle()) }))
	}

}

impl FromKObject for IFinalOutputRecorderManager {
	unsafe fn from_kobject(obj: KObject) -> IFinalOutputRecorderManager {
		IFinalOutputRecorderManager(Session::from_kobject(obj))
	}
}
