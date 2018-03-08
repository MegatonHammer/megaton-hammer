
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IPowerStateInterface(Session);

impl IPowerStateInterface {
	pub fn get_service() -> Result<IPowerStateInterface> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"spsm\0\0\0\0").map(|s| unsafe { IPowerStateInterface::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl IPowerStateInterface {
	pub fn GetState(&self, ) -> Result<u32> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SleepSystemAndWaitAwake(&self, ) -> Result<KObject> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn Unknown2(&self, ) -> Result<u32> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn Unknown3(&self, unk0: u8) -> Result<()> {
		let req = Request::new(3)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown4(&self, ) -> Result<KObject> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn Unknown5(&self, ) -> Result<u32> {
		let req = Request::new(5)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn Unknown6(&self, ) -> Result<()> {
		let req = Request::new(6)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown7(&self, ) -> Result<()> {
		let req = Request::new(7)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn Unknown8(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown9(&self, unk0: u64) -> Result<()> {
		let req = Request::new(9)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IPowerStateInterface {
	unsafe fn from_kobject(obj: KObject) -> IPowerStateInterface {
		IPowerStateInterface(Session::from_kobject(obj))
	}
}
