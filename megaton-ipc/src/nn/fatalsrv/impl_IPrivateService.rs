
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IPrivateService(Session);

impl IPrivateService {
	pub fn get_service() -> Result<IPrivateService> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"fatal:p\0").map(|s| unsafe { IPrivateService::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IPrivateService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IPrivateService {
	pub fn Unknown0(&self, ) -> Result<KObject> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

}

impl FromKObject for IPrivateService {
	unsafe fn from_kobject(obj: KObject) -> IPrivateService {
		IPrivateService(Session::from_kobject(obj))
	}
}
