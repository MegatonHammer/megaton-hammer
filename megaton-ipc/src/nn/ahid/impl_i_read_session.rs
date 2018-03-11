
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IReadSession(Session);

impl AsRef<Session> for IReadSession {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IReadSession {
	pub fn unknown0(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IReadSession {
	unsafe fn from_kobject(obj: KObject) -> IReadSession {
		IReadSession(Session::from_kobject(obj))
	}
}
