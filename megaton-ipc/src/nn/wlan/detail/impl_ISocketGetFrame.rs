
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct ISocketGetFrame(Session);

impl ISocketGetFrame {
	pub fn get_service() -> Result<ISocketGetFrame> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"wlan:sg\0").map(|s| unsafe { ISocketGetFrame::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl ISocketGetFrame {
	pub fn Unknown0(&self, ) -> Result<()> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for ISocketGetFrame {
	unsafe fn from_kobject(obj: KObject) -> ISocketGetFrame {
		ISocketGetFrame(Session::from_kobject(obj))
	}
}
