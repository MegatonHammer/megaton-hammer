
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct ISocket<T>(T);

impl ISocket<Session> {
	pub fn to_domain(self) -> ::core::result::Result<ISocket<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ISocket(domain)),
			Err((sess, err)) => Err((ISocket(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ISocket<Session>> {
		Ok(ISocket(self.0.duplicate()?))
	}
}

impl<T> Deref for ISocket<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ISocket<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ISocket<T> {
	pub fn close(&self, ) -> Result<(u32, u32)> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk0: u32,
			unk1: u32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk0.clone(),res.get_raw().unk1.clone()))
	}

	// fn connect(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn bind(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn listen(&self, unk0: u32) -> Result<(u32, u32)> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(3)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: u32,
			unk2: u32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().unk2.clone()))
	}

	// fn accept(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn recv(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn send(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn shutdown(&self, unk0: u32) -> Result<(u32, u32)> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(7)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: u32,
			unk2: u32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().unk2.clone()))
	}

	pub fn fcntl(&self, unk0: u32, unk1: u32) -> Result<(u32, u32)> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u32,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(8)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk2: u32,
			unk3: u32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk2.clone(),res.get_raw().unk3.clone()))
	}

}

impl<T: Object> From<T> for ISocket<T> {
	fn from(obj: T) -> ISocket<T> {
		ISocket(obj)
	}
}
