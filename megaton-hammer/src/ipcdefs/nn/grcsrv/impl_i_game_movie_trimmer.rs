
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IGameMovieTrimmer<T>(T);

impl IGameMovieTrimmer<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IGameMovieTrimmer<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IGameMovieTrimmer(domain)),
			Err((sess, err)) => Err((IGameMovieTrimmer(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IGameMovieTrimmer<Session>> {
		Ok(IGameMovieTrimmer(self.0.duplicate()?))
	}
}

impl<T> Deref for IGameMovieTrimmer<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IGameMovieTrimmer<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IGameMovieTrimmer<T> {
	pub fn begin_trim(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn end_trim(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_not_trimming_event(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(10)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_thumbnail_rgba(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(20)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IGameMovieTrimmer<T> {
	fn from(obj: T) -> IGameMovieTrimmer<T> {
		IGameMovieTrimmer(obj)
	}
}
