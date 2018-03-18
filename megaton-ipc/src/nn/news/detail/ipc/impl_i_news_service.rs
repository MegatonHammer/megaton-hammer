
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct INewsService(Session);

impl AsRef<Session> for INewsService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl INewsService {
	// fn unknown10100(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown20100(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown30100(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown30101(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown30200(&self, ) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(30200)
			.args(())
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn unknown30300(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown30400(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown40100(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown40101(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(40101)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown40200(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(40200)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown40201(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(40201)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown90100(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for INewsService {
	unsafe fn from_kobject(obj: KObject) -> INewsService {
		INewsService(Session::from_kobject(obj))
	}
}
