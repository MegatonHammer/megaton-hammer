
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct INewsService(Session);

impl INewsService {
	// fn Unknown10100(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown20100(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown30100(&self, ) -> Result<()> {
		let req = Request::new(30100)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn Unknown30101(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown30200(&self, ) -> Result<(u8)> {
		let req = Request::new(30200)
			.args(())
			;
		let mut res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn Unknown30300(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown30400(&self, ) -> Result<()> {
		let req = Request::new(30400)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown40100(&self, ) -> Result<()> {
		let req = Request::new(40100)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown40101(&self, unk0: u64) -> Result<()> {
		let req = Request::new(40101)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown40200(&self, ) -> Result<()> {
		let req = Request::new(40200)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown40201(&self, ) -> Result<()> {
		let req = Request::new(40201)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn Unknown90100(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for INewsService {
	unsafe fn from_kobject(obj: KObject) -> INewsService {
		INewsService(Session::from_kobject(obj))
	}
}
