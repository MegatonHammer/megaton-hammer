
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hamer::ipc::IPCBuffer

#[derive(Debug)]
pub struct IDeliveryCacheDirectoryService(Session);

impl AsRef<Session> for IDeliveryCacheDirectoryService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IDeliveryCacheDirectoryService {
	pub fn Open(&self, unk0: ::nn::bcat::DirectoryName) -> Result<()> {
		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn Read(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetCount(&self, ) -> Result<i32> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IDeliveryCacheDirectoryService {
	unsafe fn from_kobject(obj: KObject) -> IDeliveryCacheDirectoryService {
		IDeliveryCacheDirectoryService(Session::from_kobject(obj))
	}
}
