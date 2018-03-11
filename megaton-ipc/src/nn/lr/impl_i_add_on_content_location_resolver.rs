
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IAddOnContentLocationResolver(Session);

impl AsRef<Session> for IAddOnContentLocationResolver {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IAddOnContentLocationResolver {
	pub fn get_add_on_content_nca_path(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn register_add_on_content(&self, unk0: u8, unk1: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u8,
			unk1: u64,
		}
		let req = Request::new(1)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn clear_add_on_content_location_resolver(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IAddOnContentLocationResolver {
	unsafe fn from_kobject(obj: KObject) -> IAddOnContentLocationResolver {
		IAddOnContentLocationResolver(Session::from_kobject(obj))
	}
}
