
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IAsyncProgressResult(Session);

impl AsRef<Session> for IAsyncProgressResult {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IAsyncProgressResult {
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

	pub fn Unknown2(&self, ) -> Result<u128> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<u128> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IAsyncProgressResult {
	unsafe fn from_kobject(obj: KObject) -> IAsyncProgressResult {
		IAsyncProgressResult(Session::from_kobject(obj))
	}
}
