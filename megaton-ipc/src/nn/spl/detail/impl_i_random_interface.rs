
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IRandomInterface(Session);

impl IRandomInterface {
	pub fn new() -> Result<IRandomInterface> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"csrng\0\0\0").map(|s| unsafe { IRandomInterface::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IRandomInterface {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IRandomInterface {
	// fn get_random_bytes(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IRandomInterface {
	unsafe fn from_kobject(obj: KObject) -> IRandomInterface {
		IRandomInterface(Session::from_kobject(obj))
	}
}
