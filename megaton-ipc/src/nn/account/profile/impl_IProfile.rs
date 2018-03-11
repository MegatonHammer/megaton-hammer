
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hamer::ipc::IPCBuffer

#[derive(Debug)]
pub struct IProfile(Session);

impl AsRef<Session> for IProfile {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IProfile {
	// fn Get(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetBase(&self, ) -> Result<::nn::account::profile::ProfileBase> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<::nn::account::profile::ProfileBase> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetImageSize(&self, ) -> Result<u32> {
		let req = Request::new(10)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn LoadImage(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IProfile {
	unsafe fn from_kobject(obj: KObject) -> IProfile {
		IProfile(Session::from_kobject(obj))
	}
}
