
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IBcatService(Session);

impl AsRef<Session> for IBcatService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IBcatService {
	pub fn request_sync_delivery_cache(&self, ) -> Result<::nn::bcat::detail::ipc::IDeliveryCacheProgressService> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10100)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn request_sync_delivery_cache_with_application_id(&self, unk0: u32, unk1: ::nn::ApplicationId) -> Result<::nn::bcat::detail::ipc::IDeliveryCacheProgressService> {
		use megaton_hammer::ipc::{Request, Response};

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

	// fn set_passphrase(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn register_background_delivery_task(&self, unk0: u32, unk1: ::nn::ApplicationId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unregister_background_delivery_task(&self, unk0: ::nn::ApplicationId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(30201)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn block_delivery_task(&self, unk0: ::nn::ApplicationId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(30202)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unblock_delivery_task(&self, unk0: ::nn::ApplicationId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(30203)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn enumerate_background_delivery_task(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_delivery_list(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn clear_delivery_cache_storage(&self, unk0: ::nn::ApplicationId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(90201)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_push_notification_log(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IBcatService {
	unsafe fn from_kobject(obj: KObject) -> IBcatService {
		IBcatService(Session::from_kobject(obj))
	}
}
