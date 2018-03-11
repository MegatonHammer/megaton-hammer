
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct ILocalGetFrame(Session);

impl ILocalGetFrame {
	pub fn new() -> Result<ILocalGetFrame> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"wlan:lg\0").map(|s| unsafe { ILocalGetFrame::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for ILocalGetFrame {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ILocalGetFrame {
	pub fn unknown0(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for ILocalGetFrame {
	unsafe fn from_kobject(obj: KObject) -> ILocalGetFrame {
		ILocalGetFrame(Session::from_kobject(obj))
	}
}
