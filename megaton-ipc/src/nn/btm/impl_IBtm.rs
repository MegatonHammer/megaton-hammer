
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IBtm(Session);

impl IBtm {
	pub fn get_service() -> Result<IBtm> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"btm\0\0\0\0\0").map(|s| unsafe { IBtm::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl IBtm {
	pub fn Unknown0(&self, ) -> Result<u32> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
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

	pub fn Unknown4(&self, ) -> Result<()> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown5(&self, ) -> Result<()> {
		let req = Request::new(5)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown6(&self, unk0: u32) -> Result<()> {
		let req = Request::new(6)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown7(&self, unk0: u32) -> Result<()> {
		let req = Request::new(7)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown8(&self, ) -> Result<()> {
		let req = Request::new(8)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown9(&self, ) -> Result<()> {
		let req = Request::new(9)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown10(&self, ) -> Result<()> {
		let req = Request::new(10)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn Unknown11(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown12(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown13(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown14(&self, ) -> Result<()> {
		let req = Request::new(14)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown15(&self, ) -> Result<()> {
		let req = Request::new(15)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn Unknown16(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown17(&self, ) -> Result<()> {
		let req = Request::new(17)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown18(&self, ) -> Result<()> {
		let req = Request::new(18)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IBtm {
	unsafe fn from_kobject(obj: KObject) -> IBtm {
		IBtm(Session::from_kobject(obj))
	}
}
