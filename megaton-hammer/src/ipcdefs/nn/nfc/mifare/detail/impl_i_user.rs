
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IUser<T>(T);

impl IUser<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IUser<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IUser(domain)),
			Err((sess, err)) => Err((IUser(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IUser<Session>> {
		Ok(IUser(self.0.duplicate()?))
	}
}

impl<T> Deref for IUser<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IUser<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IUser<T> {
	// fn unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown1(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown3(&self, unk0: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(3)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown4(&self, unk0: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(4)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown5(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown6(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown7(&self, unk0: u64, unk1: &mut [u8; 0x58]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(7)
			.args(unk0)
			.descriptor(IPCBuffer::from_mut_ref(unk1, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown8(&self, unk0: u64) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(8)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn unknown9(&self, unk0: u64) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(9)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn unknown10(&self, ) -> Result<u32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(10)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn unknown11(&self, unk0: u64) -> Result<u32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(11)
			.args(unk0)
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn unknown12(&self, unk0: u64) -> Result<u32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(12)
			.args(unk0)
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn unknown13(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(13)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

}

impl<T: Object> From<T> for IUser<T> {
	fn from(obj: T) -> IUser<T> {
		IUser(obj)
	}
}
