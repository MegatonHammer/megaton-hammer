
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IManager(Session);

impl IManager {
	pub fn get_service() -> Result<IManager> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"apm\0\0\0\0\0").map(|s| unsafe { IManager::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl IManager {
	pub fn OpenSession(&self, ) -> Result<::nn::apm::ISession> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn GetPerformanceMode(&self, ) -> Result<::nn::apm::PerformanceMode> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<::nn::apm::PerformanceMode> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IManager {
	unsafe fn from_kobject(obj: KObject) -> IManager {
		IManager(Session::from_kobject(obj))
	}
}
