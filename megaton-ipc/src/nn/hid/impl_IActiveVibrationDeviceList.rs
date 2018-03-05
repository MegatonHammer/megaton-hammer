
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IActiveVibrationDeviceList(Session);

impl IActiveVibrationDeviceList {
	pub fn ActivateVibrationDevice(&self, unk0: ::nn::hid::VibrationDeviceHandle) -> Result<()> {
		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
}

impl FromKObject for IActiveVibrationDeviceList {
	unsafe fn from_kobject(obj: KObject) -> IActiveVibrationDeviceList {
		IActiveVibrationDeviceList(Session::from_kobject(obj))
	}
}
