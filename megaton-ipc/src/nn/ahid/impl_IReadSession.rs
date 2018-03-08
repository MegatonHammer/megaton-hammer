
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IReadSession(Session);

impl IReadSession {
	pub fn Unknown0(&self, ) -> Result<()> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IReadSession {
	unsafe fn from_kobject(obj: KObject) -> IReadSession {
		IReadSession(Session::from_kobject(obj))
	}
}
