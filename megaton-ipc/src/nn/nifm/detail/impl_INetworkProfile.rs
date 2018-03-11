
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hamer::ipc::IPCBuffer

#[derive(Debug)]
pub struct INetworkProfile(Session);

impl AsRef<Session> for INetworkProfile {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl INetworkProfile {
	// fn Update(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn PersistOld(&self, unk0: ::nn::util::Uuid) -> Result<::nn::util::Uuid> {
		let req = Request::new(1)
			.args(unk0)
			;
		let mut res : Response<::nn::util::Uuid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn Persist(&self, ) -> Result<::nn::util::Uuid> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<::nn::util::Uuid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for INetworkProfile {
	unsafe fn from_kobject(obj: KObject) -> INetworkProfile {
		INetworkProfile(Session::from_kobject(obj))
	}
}
