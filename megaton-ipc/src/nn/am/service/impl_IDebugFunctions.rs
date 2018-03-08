
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IDebugFunctions(Session);

impl IDebugFunctions {
	pub fn NotifyMessageToHomeMenuForDebug(&self, unk0: ::nn::am::AppletMessage) -> Result<()> {
		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn OpenMainApplication(&self, ) -> Result<(::nn::am::service::IApplicationAccessor)> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn EmulateButtonEvent(&self, unk0: ::nn::am::service::EmulatedButtonEvent) -> Result<()> {
		let req = Request::new(10)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn InvalidateTransitionLayer(&self, ) -> Result<()> {
		let req = Request::new(20)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IDebugFunctions {
	unsafe fn from_kobject(obj: KObject) -> IDebugFunctions {
		IDebugFunctions(Session::from_kobject(obj))
	}
}
