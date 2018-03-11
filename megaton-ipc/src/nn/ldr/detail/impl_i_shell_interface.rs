
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IShellInterface(Session);

impl IShellInterface {
	pub fn new() -> Result<IShellInterface> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"ldr:shel").map(|s| unsafe { IShellInterface::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IShellInterface {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IShellInterface {
	pub fn add_process_to_launch_queue(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn clear_launch_queue(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IShellInterface {
	unsafe fn from_kobject(obj: KObject) -> IShellInterface {
		IShellInterface(Session::from_kobject(obj))
	}
}
