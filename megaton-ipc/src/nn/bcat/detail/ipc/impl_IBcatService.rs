
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IBcatService(Session);

impl AsRef<Session> for IBcatService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IBcatService {
	pub fn RequestSyncDeliveryCache(&self, ) -> Result<::nn::bcat::detail::ipc::IDeliveryCacheProgressService> {
		let req = Request::new(10100)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn RequestSyncDeliveryCacheWithApplicationId(&self, unk0: u32, unk1: ::nn::ApplicationId) -> Result<::nn::bcat::detail::ipc::IDeliveryCacheProgressService> {
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

	// fn SetPassphrase(&self, UNKNOWN) -> Result<UNKNOWN>;
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

	// fn EnumerateBackgroundDeliveryTask(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetDeliveryList(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn ClearDeliveryCacheStorage(&self, unk0: ::nn::ApplicationId) -> Result<()> {
		let req = Request::new(90201)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn GetPushNotificationLog(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IBcatService {
	unsafe fn from_kobject(obj: KObject) -> IBcatService {
		IBcatService(Session::from_kobject(obj))
	}
}
