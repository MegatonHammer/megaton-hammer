
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IContentManagementInterface<T>(T);

impl IContentManagementInterface<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IContentManagementInterface<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IContentManagementInterface(domain)),
			Err((sess, err)) => Err((IContentManagementInterface(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IContentManagementInterface<Session>> {
		Ok(IContentManagementInterface(self.0.duplicate()?))
	}
}

impl<T> Deref for IContentManagementInterface<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IContentManagementInterface<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IContentManagementInterface<T> {
	// fn calculate_application_occupied_size(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn check_sd_card_mount_status(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(43)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_total_space_size(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_free_space_size(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn count_application_content_meta(&self, unk0: u64) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(600)
			.args(unk0)
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn list_application_content_meta_status(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn list_application_content_meta_status_with_rights_check(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn is_any_application_running(&self, ) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(607)
			.args(())
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl<T: Object> From<T> for IContentManagementInterface<T> {
	fn from(obj: T) -> IContentManagementInterface<T> {
		IContentManagementInterface(obj)
	}
}
