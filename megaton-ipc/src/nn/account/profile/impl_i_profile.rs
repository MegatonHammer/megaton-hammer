
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IProfile(Session);

impl AsRef<Session> for IProfile {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IProfile {
	// fn get(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_base(&self, ) -> Result<::nn::account::profile::ProfileBase> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let res : Response<::nn::account::profile::ProfileBase> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_image_size(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn load_image(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IProfile {
	unsafe fn from_kobject(obj: KObject) -> IProfile {
		IProfile(Session::from_kobject(obj))
	}
}
