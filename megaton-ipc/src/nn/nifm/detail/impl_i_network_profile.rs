
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct INetworkProfile(Session);

impl AsRef<Session> for INetworkProfile {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl INetworkProfile {
	// fn update(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn persist_old(&self, unk0: ::nn::util::Uuid) -> Result<::nn::util::Uuid> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(unk0)
			;
		let res : Response<::nn::util::Uuid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn persist(&self, ) -> Result<::nn::util::Uuid> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let res : Response<::nn::util::Uuid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for INetworkProfile {
	unsafe fn from_kobject(obj: KObject) -> INetworkProfile {
		INetworkProfile(Session::from_kobject(obj))
	}
}
