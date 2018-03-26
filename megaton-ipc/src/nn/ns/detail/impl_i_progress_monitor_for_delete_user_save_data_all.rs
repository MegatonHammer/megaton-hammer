
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IProgressMonitorForDeleteUserSaveDataAll<T>(T);

impl IProgressMonitorForDeleteUserSaveDataAll<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IProgressMonitorForDeleteUserSaveDataAll<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IProgressMonitorForDeleteUserSaveDataAll(domain)),
			Err((sess, err)) => Err((IProgressMonitorForDeleteUserSaveDataAll(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IProgressMonitorForDeleteUserSaveDataAll<Session>> {
		Ok(IProgressMonitorForDeleteUserSaveDataAll(self.0.duplicate()?))
	}
}

impl<T> Deref for IProgressMonitorForDeleteUserSaveDataAll<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IProgressMonitorForDeleteUserSaveDataAll<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IProgressMonitorForDeleteUserSaveDataAll<T> {
	pub fn unknown0(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn unknown1(&self, ) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn unknown2(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown10(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IProgressMonitorForDeleteUserSaveDataAll<T> {
	fn from(obj: T) -> IProgressMonitorForDeleteUserSaveDataAll<T> {
		IProgressMonitorForDeleteUserSaveDataAll(obj)
	}
}
