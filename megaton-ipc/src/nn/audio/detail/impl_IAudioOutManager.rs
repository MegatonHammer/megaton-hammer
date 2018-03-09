
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IAudioOutManager(Session);

impl IAudioOutManager {
	pub fn get_service() -> Result<IAudioOutManager> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"audout:u").map(|s| unsafe { IAudioOutManager::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IAudioOutManager {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IAudioOutManager {
	// fn ListAudioOuts(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn OpenAudioOut(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IAudioOutManager {
	unsafe fn from_kobject(obj: KObject) -> IAudioOutManager {
		IAudioOutManager(Session::from_kobject(obj))
	}
}
