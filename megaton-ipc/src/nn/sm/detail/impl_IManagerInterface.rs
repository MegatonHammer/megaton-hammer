
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hamer::ipc::IPCBuffer

#[derive(Debug)]
pub struct IManagerInterface(Session);

impl IManagerInterface {
	pub fn get_service() -> Result<IManagerInterface> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"sm:m\0\0\0\0").map(|s| unsafe { IManagerInterface::from_kobject(s) });
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
	// fn RegisterProcess(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn UnregisterProcess(&self, unk0: u64) -> Result<()> {
		let req = Request::new(1)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IManagerInterface {
	unsafe fn from_kobject(obj: KObject) -> IManagerInterface {
		IManagerInterface(Session::from_kobject(obj))
	}
}
