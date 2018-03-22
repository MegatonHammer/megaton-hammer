
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct ILocationResolver(Session);

impl AsRef<Session> for ILocationResolver {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ILocationResolver {
	// fn get_program_nca_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_program_nca_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_user_control_nca_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_doc_html_nca_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_control_nca_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_control_nca_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_doc_html_nca_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_info_html_nca_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_info_html_nca_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn clear_location_resolver(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(9)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for ILocationResolver {
	unsafe fn from_kobject(obj: KObject) -> ILocationResolver {
		ILocationResolver(Session::from_kobject(obj))
	}
}
