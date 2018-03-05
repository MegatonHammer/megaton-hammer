
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct ILdrShellInterface(Session);

impl ILdrShellInterface {
	pub fn AddProcessToLaunchQueue(&self, unk0: [u8; 0x200], size: u32, appID: ::nn::ncm::ApplicationId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			size: u32,
			appID: ::nn::ncm::ApplicationId,
		}
		let req = Request::new(0)
			.args(InRaw {
				size,
				appID,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
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
