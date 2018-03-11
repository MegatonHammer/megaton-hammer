
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IStorage(Session);

impl AsRef<Session> for IStorage {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IStorage {
	// fn read(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn write(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn flush(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_size(&self, size: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(size)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_size(&self, ) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IStorage {
	unsafe fn from_kobject(obj: KObject) -> IStorage {
		IStorage(Session::from_kobject(obj))
	}
}
