
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IScreenShotService(Session);

impl IScreenShotService {
	pub fn get_service() -> Result<IScreenShotService> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"caps:ss\0").map(|s| unsafe { IScreenShotService::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IScreenShotService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IScreenShotService {
	pub fn Unknown201(&self, ) -> Result<()> {
		let req = Request::new(201)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown202(&self, ) -> Result<()> {
		let req = Request::new(202)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown203(&self, ) -> Result<()> {
		let req = Request::new(203)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown204(&self, ) -> Result<()> {
		let req = Request::new(204)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IScreenShotService {
	unsafe fn from_kobject(obj: KObject) -> IScreenShotService {
		IScreenShotService(Session::from_kobject(obj))
	}
}
