
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IRequest<T>(T);

impl IRequest<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IRequest<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IRequest(domain)),
			Err((sess, err)) => Err((IRequest(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IRequest<Session>> {
		Ok(IRequest(self.0.duplicate()?))
	}
}

impl<T> Deref for IRequest<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IRequest<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IRequest<T> {
	pub fn initialize(&self, unk0: ::ipcdefs::nn::fgm::Module, unk1: u64) -> Result<KObject> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::fgm::Module,
			unk1: u64,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn set(&self, unk0: u32, unk1: u32) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u32,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get(&self, ) -> Result<u32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn cancel(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(3)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IRequest<T> {
	fn from(obj: T) -> IRequest<T> {
		IRequest(obj)
	}
}
