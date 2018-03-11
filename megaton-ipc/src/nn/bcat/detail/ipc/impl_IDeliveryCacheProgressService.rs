
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hamer::ipc::IPCBuffer

#[derive(Debug)]
pub struct IDeliveryCacheProgressService(Session);

impl AsRef<Session> for IDeliveryCacheProgressService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IDeliveryCacheProgressService {
	pub fn GetEvent(&self, ) -> Result<KObject> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn GetImpl(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IDeliveryCacheProgressService {
	unsafe fn from_kobject(obj: KObject) -> IDeliveryCacheProgressService {
		IDeliveryCacheProgressService(Session::from_kobject(obj))
	}
}
