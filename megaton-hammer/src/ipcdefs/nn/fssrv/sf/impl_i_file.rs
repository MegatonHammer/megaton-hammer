
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IFile<T>(T);

impl IFile<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IFile<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IFile(domain)),
			Err((sess, err)) => Err((IFile(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IFile<Session>> {
		Ok(IFile(self.0.duplicate()?))
	}
}

impl<T> Deref for IFile<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IFile<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IFile<T> {
	pub fn read(&self, unk0: u32, offset: u64, size: u64, out_buf: &mut [i8]) -> Result<u64> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			offset: u64,
			size: u64,
		}
		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(0)
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
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			offset: u64,
			size: u64,
		}
		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(1)
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
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_size(&self, size: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(3)
			.args(size)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_size(&self, ) -> Result<u64> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(4)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl<T: Object> From<T> for IFile<T> {
	fn from(obj: T) -> IFile<T> {
		IFile(obj)
	}
}
