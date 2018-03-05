
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IWindowController(Session);

impl IWindowController {
	pub fn CreateWindow(&self, unk0: ::nn::am::service::WindowCreationOption) -> Result<::nn::am::service::IWindow> {
		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}
	pub fn GetAppletResourceUserId(&self, ) -> Result<::nn::applet::AppletResourceUserId> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<::nn::applet::AppletResourceUserId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn AcquireForegroundRights(&self, ) -> Result<()> {
		let req = Request::new(10)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn ReleaseForegroundRights(&self, ) -> Result<()> {
		let req = Request::new(11)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn RejectToChangeIntoBackground(&self, ) -> Result<()> {
		let req = Request::new(12)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
}

impl FromKObject for IWindowController {
	unsafe fn from_kobject(obj: KObject) -> IWindowController {
		IWindowController(Session::from_kobject(obj))
	}
}
