
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IFactoryResetInterface(Session);

impl AsRef<Session> for IFactoryResetInterface {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IFactoryResetInterface {
	pub fn unknown100(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(100)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown101(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(101)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown102(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(102)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IFactoryResetInterface {
	unsafe fn from_kobject(obj: KObject) -> IFactoryResetInterface {
		IFactoryResetInterface(Session::from_kobject(obj))
	}
}
