
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IAudioInManager(Session);

impl IAudioInManager {
	pub fn get_service() -> Result<IAudioInManager> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"audin:u\0").map(|s| unsafe { IAudioInManager::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl IAudioInManager {
	// fn Unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown3(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown4(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IAudioInManager {
	unsafe fn from_kobject(obj: KObject) -> IAudioInManager {
		IAudioInManager(Session::from_kobject(obj))
	}
}
