
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IBcatService<T>(T);

impl IBcatService<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IBcatService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IBcatService(domain)),
			Err((sess, err)) => Err((IBcatService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IBcatService<Session>> {
		Ok(IBcatService(self.0.duplicate()?))
	}
}

impl<T> Deref for IBcatService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IBcatService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IBcatService<T> {
	pub fn request_sync_delivery_cache(&self, ) -> Result<::ipcdefs::nn::bcat::detail::ipc::IDeliveryCacheProgressService<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(10100)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn request_sync_delivery_cache_with_application_id(&self, unk0: u32, unk1: ::ipcdefs::nn::ApplicationId) -> Result<::ipcdefs::nn::bcat::detail::ipc::IDeliveryCacheProgressService<T>> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: ::ipcdefs::nn::ApplicationId,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(20100)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn set_passphrase(&self, unk0: ::ipcdefs::nn::ApplicationId, unk1: &[i8]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(30100)
			.args(unk0)
			.descriptor(IPCBuffer::from_slice(unk1, 9))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn register_background_delivery_task(&self, unk0: u32, unk1: ::ipcdefs::nn::ApplicationId) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: ::ipcdefs::nn::ApplicationId,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30200)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unregister_background_delivery_task(&self, unk0: ::ipcdefs::nn::ApplicationId) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30201)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn block_delivery_task(&self, unk0: ::ipcdefs::nn::ApplicationId) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30202)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unblock_delivery_task(&self, unk0: ::ipcdefs::nn::ApplicationId) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30203)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn enumerate_background_delivery_task(&self, unk1: &mut [::ipcdefs::nn::bcat::TaskInfo]) -> Result<i32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(90100)
			.args(())
			.descriptor(IPCBuffer::from_mut_slice(unk1, 6))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_delivery_list(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn clear_delivery_cache_storage(&self, unk0: ::ipcdefs::nn::ApplicationId) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(90201)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_push_notification_log(&self, unk1: &mut [::ipcdefs::nn::bcat::PushNotificationLog]) -> Result<i32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(90300)
			.args(())
			.descriptor(IPCBuffer::from_mut_slice(unk1, 6))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl<T: Object> From<T> for IBcatService<T> {
	fn from(obj: T) -> IBcatService<T> {
		IBcatService(obj)
	}
}
