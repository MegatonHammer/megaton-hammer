
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IAsyncValue(Session);

impl AsRef<Session> for IAsyncValue {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IAsyncValue {
	pub fn unknown0(&self, ) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown2(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IAsyncValue {
	unsafe fn from_kobject(obj: KObject) -> IAsyncValue {
		IAsyncValue(Session::from_kobject(obj))
	}
}
