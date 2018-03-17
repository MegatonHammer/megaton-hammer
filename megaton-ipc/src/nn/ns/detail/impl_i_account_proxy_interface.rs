
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IAccountProxyInterface(Session);

impl AsRef<Session> for IAccountProxyInterface {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IAccountProxyInterface {
	pub fn create_user_account(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IAccountProxyInterface {
	unsafe fn from_kobject(obj: KObject) -> IAccountProxyInterface {
		IAccountProxyInterface(Session::from_kobject(obj))
	}
}
