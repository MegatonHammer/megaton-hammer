
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct ISystemManager(Session);

impl ISystemManager {
	pub fn new() -> Result<ISystemManager> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"apm:sys\0").map(|s| unsafe { ISystemManager::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for ISystemManager {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ISystemManager {
	pub fn request_performance_mode(&self, unk0: ::nn::apm::PerformanceMode) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_performance_event(&self, unk0: ::nn::apm::EventTarget) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn get_throttling_state(&self, ) -> Result<::nn::apm::ThrottlingState> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let res : Response<::nn::apm::ThrottlingState> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_last_throttling_state(&self, ) -> Result<::nn::apm::ThrottlingState> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			;
		let res : Response<::nn::apm::ThrottlingState> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn clear_last_throttling_state(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for ISystemManager {
	unsafe fn from_kobject(obj: KObject) -> ISystemManager {
		ISystemManager(Session::from_kobject(obj))
	}
}
