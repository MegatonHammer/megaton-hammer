
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct INewsService<T>(T);

impl INewsService<Session> {
	pub fn to_domain(self) -> ::core::result::Result<INewsService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(INewsService(domain)),
			Err((sess, err)) => Err((INewsService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<INewsService<Session>> {
		Ok(INewsService(self.0.duplicate()?))
	}
}

impl<T> Deref for INewsService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for INewsService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> INewsService<T> {
	// fn unknown10100(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown20100(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown30100(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown30101(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown30200(&self, ) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30200)
			.args(())
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn unknown30300(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown30400(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown40100(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown40101(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(40101)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown40200(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(40200)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown40201(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(40201)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown90100(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for INewsService<T> {
	fn from(obj: T) -> INewsService<T> {
		INewsService(obj)
	}
}
