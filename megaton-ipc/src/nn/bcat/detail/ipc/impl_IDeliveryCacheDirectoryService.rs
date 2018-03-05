
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IDeliveryCacheDirectoryService(Session);

impl IDeliveryCacheDirectoryService {
	pub fn Open(&self, unk0: ::nn::bcat::DirectoryName) -> Result<()> {
		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Read(&self, unk1: &mut [::nn::bcat::DeliveryCacheDirectoryEntry]) -> Result<i32> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
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
