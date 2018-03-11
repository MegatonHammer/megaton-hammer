
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IFileSystem(Session);

impl AsRef<Session> for IFileSystem {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IFileSystem {
	// fn create_file(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn delete_file(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn create_directory(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn delete_directory(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn delete_directory_recursively(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn rename_file(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn rename_directory(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_entry_type(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn open_file(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn open_directory(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn commit(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_free_space_size(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_total_space_size(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn clean_directory_recursively(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_file_time_stamp_raw(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IFileSystem {
	unsafe fn from_kobject(obj: KObject) -> IFileSystem {
		IFileSystem(Session::from_kobject(obj))
	}
}
