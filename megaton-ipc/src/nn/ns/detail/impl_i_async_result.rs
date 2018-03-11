
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IAsyncResult(Session);

impl AsRef<Session> for IAsyncResult {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IAsyncResult {
	pub fn unknown0(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown1(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IAsyncResult {
	unsafe fn from_kobject(obj: KObject) -> IAsyncResult {
		IAsyncResult(Session::from_kobject(obj))
	}
}
