
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IFile(Session);

impl IFile {
	pub fn Read(&self, unk0: u32, offset: u64, size: u64, out_buf: &mut Option<i8>) -> Result<(u64)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			offset: u64,
			size: u64,
		}
		let req = Request::new(0)
			.args(InRaw {
				unk0,
				offset,
				size,
			})
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn Write(&self, unk0: u32, offset: u64, size: u64, buf: &i8) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			offset: u64,
			size: u64,
		}
		let req = Request::new(1)
			.args(InRaw {
				unk0,
				offset,
				size,
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

	pub fn GetSize(&self, ) -> Result<(u64)> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IFile {
	unsafe fn from_kobject(obj: KObject) -> IFile {
		IFile(Session::from_kobject(obj))
	}
}
