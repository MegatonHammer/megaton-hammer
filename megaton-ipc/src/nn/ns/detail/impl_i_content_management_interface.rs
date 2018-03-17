
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IContentManagementInterface(Session);

impl AsRef<Session> for IContentManagementInterface {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IContentManagementInterface {
	pub fn calculate_application_occupied_size(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(11)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn check_sd_card_mount_status(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(43)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_total_space_size(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(47)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_free_space_size(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(48)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn count_application_content_meta(&self, unk0: u64) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(600)
			.args(unk0)
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn list_application_content_meta_status(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(601)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn list_application_content_meta_status_with_rights_check(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(605)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_any_application_running(&self, ) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(607)
			.args(())
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IContentManagementInterface {
	unsafe fn from_kobject(obj: KObject) -> IContentManagementInterface {
		IContentManagementInterface(Session::from_kobject(obj))
	}
}
