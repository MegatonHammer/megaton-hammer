
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IHidTemporaryServer(Session);

impl IHidTemporaryServer {
	pub fn get_service() -> Result<IHidTemporaryServer> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"hid:tmp\0").map(|s| unsafe { IHidTemporaryServer::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IHidTemporaryServer {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IHidTemporaryServer {
	pub fn GetConsoleSixAxisSensorCalibrationValues(&self, unk0: ::nn::hid::ConsoleSixAxisSensorHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<::nn::hid::tmp::ConsoleSixAxisSensorCalibrationValues> {
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
