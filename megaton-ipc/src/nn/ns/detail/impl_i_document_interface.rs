
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IDocumentInterface<T>(T);

impl IDocumentInterface<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IDocumentInterface<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IDocumentInterface(domain)),
			Err((sess, err)) => Err((IDocumentInterface(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IDocumentInterface<Session>> {
		Ok(IDocumentInterface(self.0.duplicate()?))
	}
}

impl<T> Deref for IDocumentInterface<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IDocumentInterface<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IDocumentInterface<T> {
	// fn get_application_content_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn resolve_application_content_path(&self, unk0: u8, unk1: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u8,
			unk1: u64,
		}
		let req = Request::new(23)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IDocumentInterface<T> {
	fn from(obj: T) -> IDocumentInterface<T> {
		IDocumentInterface(obj)
	}
}
