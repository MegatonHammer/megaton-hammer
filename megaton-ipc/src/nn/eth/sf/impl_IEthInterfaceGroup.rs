
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IEthInterfaceGroup(Session);

impl IEthInterfaceGroup {
	pub fn get_service() -> Result<IEthInterfaceGroup> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"ethc:i\0\0").map(|s| unsafe { IEthInterfaceGroup::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl IEthInterfaceGroup {
	pub fn Unknown0(&self, ) -> Result<KObject> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn Unknown1(&self, ) -> Result<()> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown2(&self, ) -> Result<()> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown3(&self, ) -> Result<()> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown4(&self, ) -> Result<u32> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IEthInterfaceGroup {
	unsafe fn from_kobject(obj: KObject) -> IEthInterfaceGroup {
		IEthInterfaceGroup(Session::from_kobject(obj))
	}
}
