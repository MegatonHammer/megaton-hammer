
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hamer::ipc::IPCBuffer

#[derive(Debug)]
pub struct ILdrShellInterface(Session);

impl ILdrShellInterface {
	pub fn get_service() -> Result<ILdrShellInterface> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"ldr:shel").map(|s| unsafe { ILdrShellInterface::from_kobject(s) });
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
	// fn AddProcessToLaunchQueue(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn ClearLaunchQueue(&self, ) -> Result<()> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for ILdrShellInterface {
	unsafe fn from_kobject(obj: KObject) -> ILdrShellInterface {
		ILdrShellInterface(Session::from_kobject(obj))
	}
}
