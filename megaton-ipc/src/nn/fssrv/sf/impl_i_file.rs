
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IFile(Session);

impl AsRef<Session> for IFile {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IFile {
	pub fn read(&self, unk0: u32, offset: u64, size: u64, out_buf: &mut [i8]) -> Result<u64> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

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
			.descriptor(IPCBuffer::from_mut_slice(out_buf, 0x46))
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn write(&self, unk0: u32, offset: u64, size: u64, buf: &[i8]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

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
			.descriptor(IPCBuffer::from_slice(buf, 0x45))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

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

impl FromKObject for IFile {
	unsafe fn from_kobject(obj: KObject) -> IFile {
		IFile(Session::from_kobject(obj))
	}
}
