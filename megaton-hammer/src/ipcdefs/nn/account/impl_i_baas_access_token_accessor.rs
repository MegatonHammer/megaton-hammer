
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IBaasAccessTokenAccessor<T>(T);

impl IBaasAccessTokenAccessor<Session> {
	pub fn raw_new() -> Result<IBaasAccessTokenAccessor<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"acc:aa\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IBaasAccessTokenAccessor<Session>>> {
		use alloc::arc::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IBaasAccessTokenAccessor<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"acc:aa\0\0") {
			let ret = Arc::new(IBaasAccessTokenAccessor(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IBaasAccessTokenAccessor<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IBaasAccessTokenAccessor(domain)),
			Err((sess, err)) => Err((IBaasAccessTokenAccessor(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IBaasAccessTokenAccessor<Session>> {
		Ok(IBaasAccessTokenAccessor(self.0.duplicate()?))
	}
}

impl<T> Deref for IBaasAccessTokenAccessor<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IBaasAccessTokenAccessor<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IBaasAccessTokenAccessor<T> {
	pub fn ensure_cache_async(&self, unk0: ::ipcdefs::nn::account::Uid) -> Result<::ipcdefs::nn::account::detail::IAsyncContext<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	// fn load_cache(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_device_account_id(&self, unk0: ::ipcdefs::nn::account::Uid) -> Result<u64> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(unk0)
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn register_notification_token_async(&self, unk0: ::ipcdefs::nn::npns::NotificationToken, unk1: ::ipcdefs::nn::account::Uid) -> Result<::ipcdefs::nn::account::detail::IAsyncContext<T>> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::npns::NotificationToken,
			unk1: ::ipcdefs::nn::account::Uid,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(50)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn unregister_notification_token_async(&self, unk0: ::ipcdefs::nn::account::Uid) -> Result<::ipcdefs::nn::account::detail::IAsyncContext<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(51)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

}

impl<T: Object> From<T> for IBaasAccessTokenAccessor<T> {
	fn from(obj: T) -> IBaasAccessTokenAccessor<T> {
		IBaasAccessTokenAccessor(obj)
	}
}
