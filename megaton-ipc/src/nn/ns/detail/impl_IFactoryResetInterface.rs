
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IFactoryResetInterface(Session);

impl AsRef<Session> for IFactoryResetInterface {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IFactoryResetInterface {
	pub fn Unknown100(&self, ) -> Result<()> {
		let req = Request::new(100)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown101(&self, ) -> Result<()> {
		let req = Request::new(101)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown102(&self, ) -> Result<()> {
		let req = Request::new(102)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IFactoryResetInterface {
	unsafe fn from_kobject(obj: KObject) -> IFactoryResetInterface {
		IFactoryResetInterface(Session::from_kobject(obj))
	}
}
