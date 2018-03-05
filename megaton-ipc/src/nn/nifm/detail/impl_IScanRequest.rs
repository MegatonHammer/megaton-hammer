
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IScanRequest(Session);

impl IScanRequest {
	pub fn Submit(&self, ) -> Result<()> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn IsProcessing(&self, ) -> Result<bool> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetResult(&self, ) -> Result<()> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn GetSystemEventReadableHandle(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IScanRequest {
	unsafe fn from_kobject(obj: KObject) -> IScanRequest {
		IScanRequest(Session::from_kobject(obj))
	}
}
