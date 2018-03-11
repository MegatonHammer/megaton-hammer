
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IAudioInManager(Session);

impl IAudioInManager {
	pub fn new() -> Result<IAudioInManager> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"audin:u\0").map(|s| unsafe { IAudioInManager::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IAudioInManager {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IAudioInManager {
	// fn unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown3(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown4(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IAudioInManager {
	unsafe fn from_kobject(obj: KObject) -> IAudioInManager {
		IAudioInManager(Session::from_kobject(obj))
	}
}
