
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IEventNotifier(Session);

impl AsRef<Session> for IEventNotifier {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IEventNotifier {
	pub fn BindEvent(&self, ) -> Result<KObject> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

}

impl FromKObject for IEventNotifier {
	unsafe fn from_kobject(obj: KObject) -> IEventNotifier {
		IEventNotifier(Session::from_kobject(obj))
	}
}
