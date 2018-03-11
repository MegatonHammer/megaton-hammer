
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IAsyncProgressResult(Session);

impl AsRef<Session> for IAsyncProgressResult {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IAsyncProgressResult {
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

	pub fn unknown2(&self, ) -> Result<u128> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let res : Response<u128> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IAsyncProgressResult {
	unsafe fn from_kobject(obj: KObject) -> IAsyncProgressResult {
		IAsyncProgressResult(Session::from_kobject(obj))
	}
}
