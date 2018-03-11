
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hammer::ipc::IPCBuffer;

#[derive(Debug)]
pub struct IScreenShotApplicationService(Session);

impl IScreenShotApplicationService {
	pub fn get_service() -> Result<IScreenShotApplicationService> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"caps:su\0").map(|s| unsafe { IScreenShotApplicationService::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IScreenShotApplicationService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IScreenShotApplicationService {
	// fn SaveScreenShot(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn SaveScreenShotEx0(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IScreenShotApplicationService {
	unsafe fn from_kobject(obj: KObject) -> IScreenShotApplicationService {
		IScreenShotApplicationService(Session::from_kobject(obj))
	}
}
