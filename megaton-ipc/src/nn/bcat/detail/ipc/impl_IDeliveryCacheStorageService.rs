
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IDeliveryCacheStorageService(Session);

impl IDeliveryCacheStorageService {
	pub fn CreateFileService(&self, ) -> Result<(::nn::bcat::detail::ipc::IDeliveryCacheFileService)> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn CreateDirectoryService(&self, ) -> Result<(::nn::bcat::detail::ipc::IDeliveryCacheDirectoryService)> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn EnumerateDeliveryCacheDirectory(&self, unk1: &mut [::nn::bcat::DirectoryName]) -> Result<(i32)> {
		let req = Request::new(10)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IDeliveryCacheStorageService {
	unsafe fn from_kobject(obj: KObject) -> IDeliveryCacheStorageService {
		IDeliveryCacheStorageService(Session::from_kobject(obj))
	}
}
