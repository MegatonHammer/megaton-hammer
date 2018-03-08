
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IStorage(Session);

impl IStorage {
	pub fn Read(&self, offset: u64, length: u64, buffer: &mut Option<i8>) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			offset: u64,
			length: u64,
		}
		let req = Request::new(0)
			.args(InRaw {
				offset,
				length,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Write(&self, offset: u64, length: u64, data: &i8) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			offset: u64,
			length: u64,
		}
		let req = Request::new(1)
			.args(InRaw {
				offset,
				length,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Flush(&self, ) -> Result<()> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetSize(&self, size: u64) -> Result<()> {
		let req = Request::new(3)
			.args(size)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetSize(&self, ) -> Result<u64> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IStorage {
	unsafe fn from_kobject(obj: KObject) -> IStorage {
		IStorage(Session::from_kobject(obj))
	}
}
