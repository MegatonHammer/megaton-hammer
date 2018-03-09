
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IAsyncResult(Session);

impl AsRef<Session> for IAsyncResult {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IAsyncResult {
	pub fn Unknown0(&self, ) -> Result<()> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown1(&self, ) -> Result<()> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IAsyncResult {
	unsafe fn from_kobject(obj: KObject) -> IAsyncResult {
		IAsyncResult(Session::from_kobject(obj))
	}
}
