
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IApplicationAccessor(Session);

impl AsRef<Session> for IApplicationAccessor {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IApplicationAccessor {
	pub fn get_applet_state_changed_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn is_completed(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn start(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn request_exit(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(20)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn terminate(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(25)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_result(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(30)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn request_for_application_to_get_foreground(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(101)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn terminate_all_library_applets(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(110)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn are_any_library_applets_left(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(111)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_current_library_applet(&self, ) -> Result<::nn::am::service::IAppletAccessor> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(112)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn get_application_id(&self, ) -> Result<::nn::ncm::ApplicationId> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(120)
			.args(())
			;
		let res : Response<::nn::ncm::ApplicationId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn push_launch_parameter(&self, unk0: u32, unk1: &::nn::am::service::IStorage) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(121)
			.args(unk0)
			.copy_handle(unk1.as_ref().as_ref())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_application_control_property(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_application_launch_property(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IApplicationAccessor {
	unsafe fn from_kobject(obj: KObject) -> IApplicationAccessor {
		IApplicationAccessor(Session::from_kobject(obj))
	}
}
