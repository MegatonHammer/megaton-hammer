
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IUserServiceCreator(Session);

impl IUserServiceCreator {
	pub fn get_service() -> Result<IUserServiceCreator> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"ldn:u\0\0\0").map(|s| unsafe { IUserServiceCreator::from_kobject(s) });
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
	pub fn GetUserLocalCommunicationService(&self, ) -> Result<Session> {
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
