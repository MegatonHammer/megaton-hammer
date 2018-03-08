
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
		let r = sm.GetService(*b"uart\0\0\0\0").map(|s| unsafe { IManager::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl IManager {
	pub fn DoesUartExist(&self, unk0: u32) -> Result<u8> {
		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn DoesUartExistForTest(&self, unk0: u32) -> Result<u8> {
		let req = Request::new(1)
			.args(unk0)
			;
		let mut res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SetUartBaudrate(&self, ) -> Result<()> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetUartBaudrateForTest(&self, ) -> Result<()> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn IsSomethingUartValid(&self, ) -> Result<()> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn IsSomethingUartValidForTest(&self, ) -> Result<()> {
		let req = Request::new(5)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetSession(&self, ) -> Result<Session> {
		let req = Request::new(6)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn IsSomethingUartValid2(&self, ) -> Result<()> {
		let req = Request::new(7)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn IsSomethingUartValid2ForTest(&self, ) -> Result<()> {
		let req = Request::new(8)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IManager {
	unsafe fn from_kobject(obj: KObject) -> IManager {
		IManager(Session::from_kobject(obj))
	}
}
