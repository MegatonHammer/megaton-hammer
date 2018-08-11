
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
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
	// fn resolve_program_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn redirect_program_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn resolve_application_control_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn resolve_application_html_document_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn resolve_data_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn redirect_application_control_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn redirect_application_html_document_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn resolve_application_legal_information_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn redirect_application_legal_information_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn refresh(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(9)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn set_program_nca_path2(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(10)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn clear_location_resolver2(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(11)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn delete_program_nca_path(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(12)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn delete_control_nca_path(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(13)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn delete_doc_html_nca_path(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(14)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn delete_info_html_nca_path(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(15)
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
