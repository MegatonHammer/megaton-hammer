
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct INpnsUser(Session);

impl INpnsUser {
	pub fn get_service() -> Result<INpnsUser> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"npns:u\0\0").map(|s| unsafe { INpnsUser::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl INpnsUser {
	pub fn Unknown1(&self, ) -> Result<()> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown2(&self, unk0: u64) -> Result<()> {
		let req = Request::new(2)
			.args(unk0)
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

	pub fn Unknown5(&self, ) -> Result<KObject> {
		let req = Request::new(5)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn Unknown7(&self, ) -> Result<KObject> {
		let req = Request::new(7)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn Unknown21(&self, ) -> Result<()> {
		let req = Request::new(21)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown23(&self, ) -> Result<()> {
		let req = Request::new(23)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown25(&self, ) -> Result<()> {
		let req = Request::new(25)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown101(&self, ) -> Result<()> {
		let req = Request::new(101)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown102(&self, ) -> Result<()> {
		let req = Request::new(102)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown103(&self, ) -> Result<u32> {
		let req = Request::new(103)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn Unknown104(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown111(&self, ) -> Result<()> {
		let req = Request::new(111)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for INpnsUser {
	unsafe fn from_kobject(obj: KObject) -> INpnsUser {
		INpnsUser(Session::from_kobject(obj))
	}
}
