
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IAppletResource(Session);

impl AsRef<Session> for IAppletResource {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IAppletResource {
	pub fn get_shared_memory_handle(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

}

impl FromKObject for IAppletResource {
	unsafe fn from_kobject(obj: KObject) -> IAppletResource {
		IAppletResource(Session::from_kobject(obj))
	}
}
