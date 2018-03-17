
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IHtcManager(Session);

impl IHtcManager {
	pub fn new() -> Result<IHtcManager> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"htc\0\0\0\0\0").map(|s| unsafe { IHtcManager::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IHtcManager {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IHtcManager {
	// fn get_environment_variable(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_environment_variable_length(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn bind_host_connection_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn bind_host_disconnection_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn bind_host_connection_event_for_system(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn bind_host_disconnection_event_for_system(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(5)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

}

impl FromKObject for IHtcManager {
	unsafe fn from_kobject(obj: KObject) -> IHtcManager {
		IHtcManager(Session::from_kobject(obj))
	}
}
