
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct INcmInterface5Unknown(Session);

impl INcmInterface5Unknown {
	pub fn Unknown5(&self, ) -> Result<u64> {
		let req = Request::new(5)
			.args(())
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn Unknown7(&self, ) -> Result<u64> {
		let req = Request::new(7)
			.args(())
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn Unknown8(&self, ) -> Result<()> {
		let req = Request::new(8)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown15(&self, ) -> Result<()> {
		let req = Request::new(15)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
}

impl FromKObject for INcmInterface5Unknown {
	unsafe fn from_kobject(obj: KObject) -> INcmInterface5Unknown {
		INcmInterface5Unknown(Session::from_kobject(obj))
	}
}
