
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IHardwareOpusDecoderManager(Session);

impl IHardwareOpusDecoderManager {
	// fn Unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown1(&self, unk0: u64) -> Result<u32> {
		let req = Request::new(1)
			.args(unk0)
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	// fn Unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown3(&self, unk0: [u8; 0x110]) -> Result<u32> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
}

impl FromKObject for IHardwareOpusDecoderManager {
	unsafe fn from_kobject(obj: KObject) -> IHardwareOpusDecoderManager {
		IHardwareOpusDecoderManager(Session::from_kobject(obj))
	}
}
