
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct ILdrShellInterface(Session);

impl ILdrShellInterface {
	pub fn new() -> Result<ILdrShellInterface> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"ldr:shel").map(|s| unsafe { ILdrShellInterface::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for ILdrShellInterface {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ILdrShellInterface {
	// fn add_process_to_launch_queue(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn clear_launch_queue(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for ILdrShellInterface {
	unsafe fn from_kobject(obj: KObject) -> ILdrShellInterface {
		ILdrShellInterface(Session::from_kobject(obj))
	}
}
