
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IScreenShotApplicationService(Session);

impl IScreenShotApplicationService {
	pub fn new() -> Result<IScreenShotApplicationService> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"caps:su\0").map(|s| unsafe { IScreenShotApplicationService::from_kobject(s) });
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
	// fn save_screen_shot(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn save_screen_shot_ex0(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IScreenShotApplicationService {
	unsafe fn from_kobject(obj: KObject) -> IScreenShotApplicationService {
		IScreenShotApplicationService(Session::from_kobject(obj))
	}
}
