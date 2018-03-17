
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IUserServiceCreator(Session);

impl IUserServiceCreator {
	pub fn new() -> Result<IUserServiceCreator> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"ldn:u\0\0\0").map(|s| unsafe { IUserServiceCreator::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IUserServiceCreator {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IUserServiceCreator {
	pub fn create_user_local_communication_service(&self, ) -> Result<Session> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IUserServiceCreator {
	unsafe fn from_kobject(obj: KObject) -> IUserServiceCreator {
		IUserServiceCreator(Session::from_kobject(obj))
	}
}
