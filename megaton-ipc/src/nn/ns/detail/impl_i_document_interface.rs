
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IDocumentInterface(Session);

impl AsRef<Session> for IDocumentInterface {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IDocumentInterface {
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

impl FromKObject for IDocumentInterface {
	unsafe fn from_kobject(obj: KObject) -> IDocumentInterface {
		IDocumentInterface(Session::from_kobject(obj))
	}
}
