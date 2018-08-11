
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IAlbumAccessorApplicationSession<T>(T);

impl IAlbumAccessorApplicationSession<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IAlbumAccessorApplicationSession<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IAlbumAccessorApplicationSession(domain)),
			Err((sess, err)) => Err((IAlbumAccessorApplicationSession(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IAlbumAccessorApplicationSession<Session>> {
		Ok(IAlbumAccessorApplicationSession(self.0.duplicate()?))
	}
}

impl<T> Deref for IAlbumAccessorApplicationSession<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IAlbumAccessorApplicationSession<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IAlbumAccessorApplicationSession<T> {
	pub fn open_album_movie_read_stream(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2001)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn close_album_movie_read_stream(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2002)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_album_movie_read_stream_movie_data_size(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2003)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn read_movie_data_from_album_movie_read_stream(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2004)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_album_movie_read_stream_broken_reason(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2005)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IAlbumAccessorApplicationSession<T> {
	fn from(obj: T) -> IAlbumAccessorApplicationSession<T> {
		IAlbumAccessorApplicationSession(obj)
	}
}
