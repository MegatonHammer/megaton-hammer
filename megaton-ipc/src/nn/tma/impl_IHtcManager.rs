
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IHtcManager(Session);

impl IHtcManager {
	// fn Unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown3(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown4(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown5(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown6(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown7(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown8(&self, unk0: u8) -> Result<()> {
		let req = Request::new(8)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
}

impl FromKObject for IHtcManager {
	unsafe fn from_kobject(obj: KObject) -> IHtcManager {
		IHtcManager(Session::from_kobject(obj))
	}
}
