
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct INotifier(Session);

impl AsRef<Session> for INotifier {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl INotifier {
	pub fn GetSystemEvent(&self, ) -> Result<KObject> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

}

impl FromKObject for INotifier {
	unsafe fn from_kobject(obj: KObject) -> INotifier {
		INotifier(Session::from_kobject(obj))
	}
}
