
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct INcmInterface4Unknown(Session);

impl AsRef<Session> for INcmInterface4Unknown {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl INcmInterface4Unknown {
	pub fn unknown10(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown13(&self, ) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(13)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for INcmInterface4Unknown {
	unsafe fn from_kobject(obj: KObject) -> INcmInterface4Unknown {
		INcmInterface4Unknown(Session::from_kobject(obj))
	}
}
