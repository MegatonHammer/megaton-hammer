
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IAddOnContentLocationResolver(Session);

impl IAddOnContentLocationResolver {
	pub fn GetAddOnContentNcaPath(&self, ) -> Result<()> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn RegisterAddOnContent(&self, unk0: u8, unk1: u64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u8,
			unk1: u64,
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
	pub fn ClearAddOnContentLocationResolver(&self, ) -> Result<()> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
}

impl FromKObject for IAddOnContentLocationResolver {
	unsafe fn from_kobject(obj: KObject) -> IAddOnContentLocationResolver {
		IAddOnContentLocationResolver(Session::from_kobject(obj))
	}
}
