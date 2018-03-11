
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct LocationResolverInterface(Session);

impl LocationResolverInterface {
	pub fn new() -> Result<LocationResolverInterface> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"lr\0\0\0\0\0\0").map(|s| unsafe { LocationResolverInterface::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for LocationResolverInterface {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl LocationResolverInterface {
}

impl FromKObject for LocationResolverInterface {
	unsafe fn from_kobject(obj: KObject) -> LocationResolverInterface {
		LocationResolverInterface(Session::from_kobject(obj))
	}
}
