
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct ILocationResolver<T>(T);

impl ILocationResolver<Session> {
	pub fn to_domain(self) -> ::core::result::Result<ILocationResolver<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ILocationResolver(domain)),
			Err((sess, err)) => Err((ILocationResolver(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ILocationResolver<Session>> {
		Ok(ILocationResolver(self.0.duplicate()?))
	}
}

impl<T> Deref for ILocationResolver<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ILocationResolver<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ILocationResolver<T> {
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

impl<T: Object> From<T> for ILocationResolver<T> {
	fn from(obj: T) -> ILocationResolver<T> {
		ILocationResolver(obj)
	}
}
