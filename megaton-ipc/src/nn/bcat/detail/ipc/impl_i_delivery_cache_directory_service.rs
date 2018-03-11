
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IDeliveryCacheDirectoryService(Session);

impl AsRef<Session> for IDeliveryCacheDirectoryService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IDeliveryCacheDirectoryService {
	pub fn open(&self, unk0: ::nn::bcat::DirectoryName) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn read(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_count(&self, ) -> Result<i32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IDeliveryCacheDirectoryService {
	unsafe fn from_kobject(obj: KObject) -> IDeliveryCacheDirectoryService {
		IDeliveryCacheDirectoryService(Session::from_kobject(obj))
	}
}
