
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IManagerInterface(Session);

impl IManagerInterface {
	pub fn new() -> Result<IManagerInterface> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"sm:m\0\0\0\0").map(|s| unsafe { IManagerInterface::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IManagerInterface {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IManagerInterface {
	// fn register_process(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unregister_process(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IManagerInterface {
	unsafe fn from_kobject(obj: KObject) -> IManagerInterface {
		IManagerInterface(Session::from_kobject(obj))
	}
}
