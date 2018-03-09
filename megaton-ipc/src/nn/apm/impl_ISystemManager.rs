
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct ISystemManager(Session);

impl ISystemManager {
	pub fn get_service() -> Result<ISystemManager> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"apm:sys\0").map(|s| unsafe { ISystemManager::from_kobject(s) });
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
	pub fn RequestPerformanceMode(&self, unk0: ::nn::apm::PerformanceMode) -> Result<()> {
		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetPerformanceEvent(&self, unk0: ::nn::apm::EventTarget) -> Result<KObject> {
		let req = Request::new(1)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn GetThrottlingState(&self, ) -> Result<::nn::apm::ThrottlingState> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<::nn::apm::ThrottlingState> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetLastThrottlingState(&self, ) -> Result<::nn::apm::ThrottlingState> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<::nn::apm::ThrottlingState> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn ClearLastThrottlingState(&self, ) -> Result<()> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for ISystemManager {
	unsafe fn from_kobject(obj: KObject) -> ISystemManager {
		ISystemManager(Session::from_kobject(obj))
	}
}
