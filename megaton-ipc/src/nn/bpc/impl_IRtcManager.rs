
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IRtcManager(Session);

impl IRtcManager {
	pub fn GetExternalRtcValue(&self, ) -> Result<(u64)> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SetExternalRtcValue(&self, unk0: u64) -> Result<()> {
		let req = Request::new(1)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ReadExternalRtcResetFlag(&self, ) -> Result<(u8)> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn ClearExternalRtcResetFlag(&self, ) -> Result<()> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IRtcManager {
	unsafe fn from_kobject(obj: KObject) -> IRtcManager {
		IRtcManager(Session::from_kobject(obj))
	}
}
