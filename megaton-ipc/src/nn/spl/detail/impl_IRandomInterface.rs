
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hammer::ipc::IPCBuffer;

#[derive(Debug)]
pub struct IRandomInterface(Session);

impl IRandomInterface {
	pub fn get_service() -> Result<IRandomInterface> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"csrng\0\0\0").map(|s| unsafe { IRandomInterface::from_kobject(s) });
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
	// fn GetRandomBytes(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IRandomInterface {
	unsafe fn from_kobject(obj: KObject) -> IRandomInterface {
		IRandomInterface(Session::from_kobject(obj))
	}
}
