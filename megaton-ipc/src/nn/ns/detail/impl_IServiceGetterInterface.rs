
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IServiceGetterInterface(Session);

impl IServiceGetterInterface {
	pub fn get_service() -> Result<IServiceGetterInterface> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"ns:rid\0\0").map(|s| unsafe { IServiceGetterInterface::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		let r = sm.GetService(*b"ns:web\0\0").map(|s| unsafe { IServiceGetterInterface::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		let r = sm.GetService(*b"ns:ec\0\0\0").map(|s| unsafe { IServiceGetterInterface::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		let r = sm.GetService(*b"ns:am2\0\0").map(|s| unsafe { IServiceGetterInterface::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		let r = sm.GetService(*b"ns:rt\0\0\0").map(|s| unsafe { IServiceGetterInterface::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl IServiceGetterInterface {
	pub fn Unknown7994(&self, ) -> Result<Session> {
		let req = Request::new(7994)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn Unknown7995(&self, ) -> Result<Session> {
		let req = Request::new(7995)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn Unknown7996(&self, ) -> Result<Session> {
		let req = Request::new(7996)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn Unknown7997(&self, ) -> Result<Session> {
		let req = Request::new(7997)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn Unknown7998(&self, ) -> Result<Session> {
		let req = Request::new(7998)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn Unknown7999(&self, ) -> Result<Session> {
		let req = Request::new(7999)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IServiceGetterInterface {
	unsafe fn from_kobject(obj: KObject) -> IServiceGetterInterface {
		IServiceGetterInterface(Session::from_kobject(obj))
	}
}
