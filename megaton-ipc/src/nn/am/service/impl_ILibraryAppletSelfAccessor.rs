
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct ILibraryAppletSelfAccessor(Session);

impl ILibraryAppletSelfAccessor {
	pub fn PopInData(&self, ) -> Result<::nn::am::service::IStorage> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn PushOutData(&self, unk0: ::nn::am::service::IStorage) -> Result<()> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn PopInteractiveInData(&self, ) -> Result<::nn::am::service::IStorage> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn PushInteractiveOutData(&self, unk0: ::nn::am::service::IStorage) -> Result<()> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetPopInDataEvent(&self, ) -> Result<KObject> {
		let req = Request::new(5)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn GetPopInteractiveInDataEvent(&self, ) -> Result<KObject> {
		let req = Request::new(6)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn ExitProcessAndReturn(&self, ) -> Result<()> {
		let req = Request::new(10)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetLibraryAppletInfo(&self, ) -> Result<::nn::am::service::LibraryAppletInfo> {
		let req = Request::new(11)
			.args(())
			;
		let mut res : Response<::nn::am::service::LibraryAppletInfo> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetMainAppletIdentityInfo(&self, ) -> Result<::nn::am::service::AppletIdentityInfo> {
		let req = Request::new(12)
			.args(())
			;
		let mut res : Response<::nn::am::service::AppletIdentityInfo> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn CanUseApplicationCore(&self, ) -> Result<bool> {
		let req = Request::new(13)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetCallerAppletIdentityInfo(&self, ) -> Result<::nn::am::service::AppletIdentityInfo> {
		let req = Request::new(14)
			.args(())
			;
		let mut res : Response<::nn::am::service::AppletIdentityInfo> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetMainAppletApplicationControlProperty(&self, unk0: &mut Option<::nn::ns::ApplicationControlProperty>) -> Result<()> {
		let req = Request::new(15)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetMainAppletStorageId(&self, ) -> Result<::nn::ncm::StorageId> {
		let req = Request::new(16)
			.args(())
			;
		let mut res : Response<::nn::ncm::StorageId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetCallerAppletIdentityInfoStack(&self, unk1: &mut [::nn::am::service::AppletIdentityInfo]) -> Result<i32> {
		let req = Request::new(17)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn PopExtraStorage(&self, ) -> Result<::nn::am::service::IStorage> {
		let req = Request::new(20)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn GetPopExtraStorageEvent(&self, ) -> Result<KObject> {
		let req = Request::new(25)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn UnpopInData(&self, unk0: ::nn::am::service::IStorage) -> Result<()> {
		let req = Request::new(30)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn UnpopExtraStorage(&self, unk0: ::nn::am::service::IStorage) -> Result<()> {
		let req = Request::new(31)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetIndirectLayerProducerHandle(&self, ) -> Result<u64> {
		let req = Request::new(40)
			.args(())
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn ReportVisibleError(&self, unk0: ::nn::err::ErrorCode) -> Result<()> {
		let req = Request::new(50)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for ILibraryAppletSelfAccessor {
	unsafe fn from_kobject(obj: KObject) -> ILibraryAppletSelfAccessor {
		ILibraryAppletSelfAccessor(Session::from_kobject(obj))
	}
}
