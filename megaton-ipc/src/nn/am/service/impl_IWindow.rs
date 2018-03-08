
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IWindow(Session);

impl IWindow {
	pub fn Unknown12345(&self, ) -> Result<()> {
		let req = Request::new(12345)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IWindow {
	unsafe fn from_kobject(obj: KObject) -> IWindow {
		IWindow(Session::from_kobject(obj))
	}
}
