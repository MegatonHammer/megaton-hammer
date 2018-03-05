
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IRequest(Session);

impl IRequest {
	// fn Initialize(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Set(&self, unk0: u32, unk1: u32) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u32,
		}
		let req = Request::new(1)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Get(&self, ) -> Result<u32> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn Cancel(&self, ) -> Result<()> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
}

impl FromKObject for IRequest {
	unsafe fn from_kobject(obj: KObject) -> IRequest {
		IRequest(Session::from_kobject(obj))
	}
}
