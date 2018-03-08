
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IBootModeInterface(Session);

impl IBootModeInterface {
	pub fn GetBootMode(&self, ) -> Result<(u32)> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SetMaintenanceBoot(&self, ) -> Result<()> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IBootModeInterface {
	unsafe fn from_kobject(obj: KObject) -> IBootModeInterface {
		IBootModeInterface(Session::from_kobject(obj))
	}
}
