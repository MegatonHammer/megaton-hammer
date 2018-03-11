
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IWindowController(Session);

impl AsRef<Session> for IWindowController {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IWindowController {
	pub fn create_window(&self, unk0: ::nn::am::service::WindowCreationOption) -> Result<::nn::am::service::IWindow> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn get_applet_resource_user_id(&self, ) -> Result<::nn::applet::AppletResourceUserId> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let res : Response<::nn::applet::AppletResourceUserId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn acquire_foreground_rights(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn release_foreground_rights(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(11)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn reject_to_change_into_background(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(12)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IWindowController {
	unsafe fn from_kobject(obj: KObject) -> IWindowController {
		IWindowController(Session::from_kobject(obj))
	}
}
