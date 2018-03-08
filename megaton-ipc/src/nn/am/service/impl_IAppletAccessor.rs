
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IAppletAccessor(Session);

impl IAppletAccessor {
	pub fn GetAppletStateChangedEvent(&self, ) -> Result<KObject> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn IsCompleted(&self, ) -> Result<bool> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn Start(&self, ) -> Result<()> {
		let req = Request::new(10)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn RequestExit(&self, ) -> Result<()> {
		let req = Request::new(20)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Terminate(&self, ) -> Result<()> {
		let req = Request::new(25)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetResult(&self, ) -> Result<()> {
		let req = Request::new(30)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IAppletAccessor {
	unsafe fn from_kobject(obj: KObject) -> IAppletAccessor {
		IAppletAccessor(Session::from_kobject(obj))
	}
}
