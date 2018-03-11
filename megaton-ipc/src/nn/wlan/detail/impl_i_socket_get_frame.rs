
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct ISocketGetFrame(Session);

impl ISocketGetFrame {
	pub fn new() -> Result<ISocketGetFrame> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"wlan:sg\0").map(|s| unsafe { ISocketGetFrame::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for ISocketGetFrame {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ISocketGetFrame {
	pub fn unknown0(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for ISocketGetFrame {
	unsafe fn from_kobject(obj: KObject) -> ISocketGetFrame {
		ISocketGetFrame(Session::from_kobject(obj))
	}
}
