
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IDebugMonitorInterface(Session);

impl IDebugMonitorInterface {
	pub fn AddProcessToDebugLaunchQueue(&self, ) -> Result<()> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ClearDebugLaunchQueue(&self, ) -> Result<()> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown2(&self, ) -> Result<()> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IDebugMonitorInterface {
	unsafe fn from_kobject(obj: KObject) -> IDebugMonitorInterface {
		IDebugMonitorInterface(Session::from_kobject(obj))
	}
}
