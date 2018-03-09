
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct LocationResolverInterface(Session);

impl LocationResolverInterface {
	pub fn get_service() -> Result<LocationResolverInterface> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"lr\0\0\0\0\0\0").map(|s| unsafe { LocationResolverInterface::from_kobject(s) });
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
