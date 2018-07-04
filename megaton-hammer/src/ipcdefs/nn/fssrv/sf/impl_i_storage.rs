
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IStorage<T>(T);

impl IStorage<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IStorage<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IStorage(domain)),
			Err((sess, err)) => Err((IStorage(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IStorage<Session>> {
		Ok(IStorage(self.0.duplicate()?))
	}
}

impl<T> Deref for IStorage<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IStorage<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IStorage<T> {
	pub fn read(&self, offset: u64, length: u64, buffer: &mut [i8]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			offset: u64,
			length: u64,
		}
		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(0)
			.args(InRaw {
				offset,
				length,
			})
			.descriptor(IPCBuffer::from_mut_slice(buffer, 0x46))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn write(&self, offset: u64, length: u64, data: &[i8]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			offset: u64,
			length: u64,
		}
		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(1)
			.args(InRaw {
				offset,
				length,
			})
			.descriptor(IPCBuffer::from_slice(data, 0x45))
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

impl<T: Object> From<T> for IStorage<T> {
	fn from(obj: T) -> IStorage<T> {
		IStorage(obj)
	}
}
