
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct ILocalGetFrame(Session);

impl ILocalGetFrame {
	pub fn get_service() -> Result<ILocalGetFrame> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"wlan:lg\0").map(|s| unsafe { ILocalGetFrame::from_kobject(s) });
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
	pub fn Unknown0(&self, ) -> Result<()> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for ILocalGetFrame {
	unsafe fn from_kobject(obj: KObject) -> ILocalGetFrame {
		ILocalGetFrame(Session::from_kobject(obj))
	}
}
