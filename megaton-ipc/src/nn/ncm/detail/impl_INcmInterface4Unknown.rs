
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct INcmInterface4Unknown(Session);

impl INcmInterface4Unknown {
	pub fn Unknown10(&self, ) -> Result<()> {
		let req = Request::new(10)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown13(&self, ) -> Result<u64> {
		let req = Request::new(13)
			.args(())
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
}

impl FromKObject for INcmInterface4Unknown {
	unsafe fn from_kobject(obj: KObject) -> INcmInterface4Unknown {
		INcmInterface4Unknown(Session::from_kobject(obj))
	}
}
