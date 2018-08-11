
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IContentMetaDatabase<T>(T);

impl IContentMetaDatabase<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IContentMetaDatabase<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IContentMetaDatabase(domain)),
			Err((sess, err)) => Err((IContentMetaDatabase(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IContentMetaDatabase<Session>> {
		Ok(IContentMetaDatabase(self.0.duplicate()?))
	}
}

impl<T> Deref for IContentMetaDatabase<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IContentMetaDatabase<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IContentMetaDatabase<T> {
	// fn set(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn remove(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_content_id_by_type(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn list_content_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn list(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_latest_content_meta_key(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn list_application(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn has(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn has_all(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_size(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_required_system_version(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_patch_id(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn disable_forcibly(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(13)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn lookup_orphan_content(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn commit(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(15)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn has_content(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn list_content_meta_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_attributes(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_required_application_version(&self, UNKNOWN) -> Result<UNKNOWN>;
	#[cfg(feature = "switch-5.0.0")]
	pub fn unknown20(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(20)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IContentMetaDatabase<T> {
	fn from(obj: T) -> IContentMetaDatabase<T> {
		IContentMetaDatabase(obj)
	}
}
