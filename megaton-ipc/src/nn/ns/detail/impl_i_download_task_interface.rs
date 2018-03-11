
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IDownloadTaskInterface(Session);

impl AsRef<Session> for IDownloadTaskInterface {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IDownloadTaskInterface {
	pub fn unknown701(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(701)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown702(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(702)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown703(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(703)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown704(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown705(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(705)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IDownloadTaskInterface {
	unsafe fn from_kobject(obj: KObject) -> IDownloadTaskInterface {
		IDownloadTaskInterface(Session::from_kobject(obj))
	}
}
