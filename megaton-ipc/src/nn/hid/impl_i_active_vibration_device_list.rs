
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IActiveVibrationDeviceList(Session);

impl AsRef<Session> for IActiveVibrationDeviceList {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IActiveVibrationDeviceList {
	pub fn activate_vibration_device(&self, unk0: ::nn::hid::VibrationDeviceHandle) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IActiveVibrationDeviceList {
	unsafe fn from_kobject(obj: KObject) -> IActiveVibrationDeviceList {
		IActiveVibrationDeviceList(Session::from_kobject(obj))
	}
}
