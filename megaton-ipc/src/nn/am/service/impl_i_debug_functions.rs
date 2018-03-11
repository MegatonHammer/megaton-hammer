
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IDebugFunctions(Session);

impl AsRef<Session> for IDebugFunctions {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IDebugFunctions {
	pub fn notify_message_to_home_menu_for_debug(&self, unk0: ::nn::am::AppletMessage) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn open_main_application(&self, ) -> Result<::nn::am::service::IApplicationAccessor> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn emulate_button_event(&self, unk0: ::nn::am::service::EmulatedButtonEvent) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn invalidate_transition_layer(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(20)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IDebugFunctions {
	unsafe fn from_kobject(obj: KObject) -> IDebugFunctions {
		IDebugFunctions(Session::from_kobject(obj))
	}
}
