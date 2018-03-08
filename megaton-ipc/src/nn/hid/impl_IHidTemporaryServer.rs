
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IHidTemporaryServer(Session);

impl IHidTemporaryServer {
	pub fn GetConsoleSixAxisSensorCalibrationValues(&self, unk0: ::nn::hid::ConsoleSixAxisSensorHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<(::nn::hid::tmp::ConsoleSixAxisSensorCalibrationValues)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::hid::ConsoleSixAxisSensorHandle,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(0)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<::nn::hid::tmp::ConsoleSixAxisSensorCalibrationValues> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IHidTemporaryServer {
	unsafe fn from_kobject(obj: KObject) -> IHidTemporaryServer {
		IHidTemporaryServer(Session::from_kobject(obj))
	}
}
