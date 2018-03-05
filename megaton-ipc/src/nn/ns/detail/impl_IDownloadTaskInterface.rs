
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IDownloadTaskInterface(Session);

impl IDownloadTaskInterface {
	pub fn Unknown701(&self, ) -> Result<()> {
		let req = Request::new(701)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown702(&self, ) -> Result<()> {
		let req = Request::new(702)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown703(&self, ) -> Result<()> {
		let req = Request::new(703)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn Unknown704(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown705(&self, ) -> Result<()> {
		let req = Request::new(705)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
}

impl FromKObject for IDownloadTaskInterface {
	unsafe fn from_kobject(obj: KObject) -> IDownloadTaskInterface {
		IDownloadTaskInterface(Session::from_kobject(obj))
	}
}
