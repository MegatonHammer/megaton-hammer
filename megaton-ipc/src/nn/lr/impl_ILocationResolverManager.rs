
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct ILocationResolverManager(Session);

impl ILocationResolverManager {
	pub fn get_service() -> Result<ILocationResolverManager> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"lr\0\0\0\0\0\0").map(|s| unsafe { ILocationResolverManager::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl ILocationResolverManager {
	pub fn GetLocationResolver(&self, ) -> Result<()> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetRegisteredLocationResolver(&self, ) -> Result<Session> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn CheckStorage(&self, unk0: u8) -> Result<()> {
		let req = Request::new(2)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetAddOnContentLocationResolver(&self, ) -> Result<Session> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for ILocationResolverManager {
	unsafe fn from_kobject(obj: KObject) -> ILocationResolverManager {
		ILocationResolverManager(Session::from_kobject(obj))
	}
}
