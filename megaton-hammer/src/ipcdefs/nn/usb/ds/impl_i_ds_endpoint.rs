
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IDsEndpoint<T>(T);

impl IDsEndpoint<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IDsEndpoint<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IDsEndpoint(domain)),
			Err((sess, err)) => Err((IDsEndpoint(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IDsEndpoint<Session>> {
		Ok(IDsEndpoint(self.0.duplicate()?))
	}
}

impl<T> Deref for IDsEndpoint<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IDsEndpoint<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IDsEndpoint<T> {
	pub fn post_buffer_async(&self, size: u32, buffer: u64) -> Result<u32> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			size: u32,
			buffer: u64,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(InRaw {
				size,
				buffer,
			})
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn unknown1(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_completion_event(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn get_report_data(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn stall(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(4)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown5(&self, unk0: u8) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(5)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IDsEndpoint<T> {
	fn from(obj: T) -> IDsEndpoint<T> {
		IDsEndpoint(obj)
	}
}
