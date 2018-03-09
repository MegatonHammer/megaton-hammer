
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct ISystemServiceCreator(Session);

impl ISystemServiceCreator {
	pub fn get_service() -> Result<ISystemServiceCreator> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"ldn:s\0\0\0").map(|s| unsafe { ISystemServiceCreator::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for ISystemServiceCreator {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ISystemServiceCreator {
	pub fn GetSystemLocalCommunicationService(&self, ) -> Result<Session> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for ISystemServiceCreator {
	unsafe fn from_kobject(obj: KObject) -> ISystemServiceCreator {
		ISystemServiceCreator(Session::from_kobject(obj))
	}
}
