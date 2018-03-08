
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IBcatService(Session);

impl IBcatService {
	pub fn RequestSyncDeliveryCache(&self, ) -> Result<(::nn::bcat::detail::ipc::IDeliveryCacheProgressService)> {
		let req = Request::new(10100)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn RequestSyncDeliveryCacheWithApplicationId(&self, unk0: u32, unk1: ::nn::ApplicationId) -> Result<(::nn::bcat::detail::ipc::IDeliveryCacheProgressService)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: ::nn::ApplicationId,
		}
		let req = Request::new(20100)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn SetPassphrase(&self, unk0: ::nn::ApplicationId, unk1: &[i8]) -> Result<()> {
		let req = Request::new(30100)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn RegisterBackgroundDeliveryTask(&self, unk0: u32, unk1: ::nn::ApplicationId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: ::nn::ApplicationId,
		}
		let req = Request::new(30200)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn UnregisterBackgroundDeliveryTask(&self, unk0: ::nn::ApplicationId) -> Result<()> {
		let req = Request::new(30201)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn BlockDeliveryTask(&self, unk0: ::nn::ApplicationId) -> Result<()> {
		let req = Request::new(30202)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn UnblockDeliveryTask(&self, unk0: ::nn::ApplicationId) -> Result<()> {
		let req = Request::new(30203)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn EnumerateBackgroundDeliveryTask(&self, unk1: &mut [::nn::bcat::TaskInfo]) -> Result<(i32)> {
		let req = Request::new(90100)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn GetDeliveryList(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn ClearDeliveryCacheStorage(&self, unk0: ::nn::ApplicationId) -> Result<()> {
		let req = Request::new(90201)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetPushNotificationLog(&self, unk1: &mut [::nn::bcat::PushNotificationLog]) -> Result<(i32)> {
		let req = Request::new(90300)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IBcatService {
	unsafe fn from_kobject(obj: KObject) -> IBcatService {
		IBcatService(Session::from_kobject(obj))
	}
}
