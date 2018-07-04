
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
	// fn insert_entry_content_records(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn read_entry_content_records(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn remove_entry_content_records(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_entry_content_nca_id(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn read_entry_content_record_entries(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn list(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_meta_record(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn list_application(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn is_entry_present(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn are_entries_present(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_entry_content_records_size(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_entry_unknown_record_size(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_entry_update_title_id(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn close_meta_database(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(13)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn check_nca_i_ds_present(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn save_meta_database(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(15)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn check_entry_has_nca_id(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn read_entry_meta_records(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_entry_unknown6(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_add_on_content_entry_unknown_record_size(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IContentMetaDatabase<T> {
	fn from(obj: T) -> IContentMetaDatabase<T> {
		IContentMetaDatabase(obj)
	}
}
