
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IContentMetaDatabase(Session);

impl AsRef<Session> for IContentMetaDatabase {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IContentMetaDatabase {
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
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(13)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn check_nca_i_ds_present(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn save_meta_database(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(15)
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

impl FromKObject for IContentMetaDatabase {
	unsafe fn from_kobject(obj: KObject) -> IContentMetaDatabase {
		IContentMetaDatabase(Session::from_kobject(obj))
	}
}
