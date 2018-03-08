
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IBtmSystem(Session);

impl IBtmSystem {
	pub fn get_service() -> Result<IBtmSystem> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"btm:sys\0").map(|s| unsafe { IBtmSystem::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl IBtmSystem {
	pub fn Unknown0(&self, ) -> Result<Session> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IBtmSystem {
	unsafe fn from_kobject(obj: KObject) -> IBtmSystem {
		IBtmSystem(Session::from_kobject(obj))
	}
}
