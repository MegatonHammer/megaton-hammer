
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct ILocalGetActionFrame(Session);

impl ILocalGetActionFrame {
	pub fn new() -> Result<ILocalGetActionFrame> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"wlan:lga").map(|s| unsafe { ILocalGetActionFrame::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for ILocalGetActionFrame {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ILocalGetActionFrame {
	pub fn unknown0(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for ILocalGetActionFrame {
	unsafe fn from_kobject(obj: KObject) -> ILocalGetActionFrame {
		ILocalGetActionFrame(Session::from_kobject(obj))
	}
}
