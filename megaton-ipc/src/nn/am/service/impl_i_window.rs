
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IWindow(Session);

impl AsRef<Session> for IWindow {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IWindow {
	pub fn unknown12345(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(12345)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IWindow {
	unsafe fn from_kobject(obj: KObject) -> IWindow {
		IWindow(Session::from_kobject(obj))
	}
}
