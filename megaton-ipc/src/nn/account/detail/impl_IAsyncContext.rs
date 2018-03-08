
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IAsyncContext(Session);

impl IAsyncContext {
	pub fn GetSystemEvent(&self, ) -> Result<KObject> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn Cancel(&self, ) -> Result<()> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn HasDone(&self, ) -> Result<bool> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetResult(&self, ) -> Result<()> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IAsyncContext {
	unsafe fn from_kobject(obj: KObject) -> IAsyncContext {
		IAsyncContext(Session::from_kobject(obj))
	}
}
