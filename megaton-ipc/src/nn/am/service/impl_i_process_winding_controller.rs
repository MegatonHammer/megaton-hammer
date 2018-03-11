
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IProcessWindingController(Session);

impl AsRef<Session> for IProcessWindingController {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IProcessWindingController {
	pub fn get_launch_reason(&self, ) -> Result<::nn::am::service::AppletProcessLaunchReason> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let res : Response<::nn::am::service::AppletProcessLaunchReason> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn open_calling_library_applet(&self, ) -> Result<::nn::am::service::ILibraryAppletAccessor> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(11)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn push_context(&self, unk0: &::nn::am::service::IStorage) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(21)
			.args(())
			.copy_handle(unk0.as_ref().as_ref())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn pop_context(&self, ) -> Result<::nn::am::service::IStorage> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(22)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn cancel_winding_reservation(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(23)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn wind_and_do_reserved(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(30)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn reserve_to_start_and_wait_and_unwind_this(&self, unk0: &::nn::am::service::ILibraryAppletAccessor) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(40)
			.args(())
			.copy_handle(unk0.as_ref().as_ref())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IProcessWindingController {
	unsafe fn from_kobject(obj: KObject) -> IProcessWindingController {
		IProcessWindingController(Session::from_kobject(obj))
	}
}
