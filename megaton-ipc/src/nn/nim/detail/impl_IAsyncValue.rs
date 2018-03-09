
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IAsyncValue(Session);

impl AsRef<Session> for IAsyncValue {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IAsyncValue {
	pub fn Unknown0(&self, ) -> Result<u64> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn Unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown2(&self, ) -> Result<()> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IAsyncValue {
	unsafe fn from_kobject(obj: KObject) -> IAsyncValue {
		IAsyncValue(Session::from_kobject(obj))
	}
}
