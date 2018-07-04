
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct ISettingsServer<T>(T);

impl ISettingsServer<Session> {
	pub fn raw_new() -> Result<ISettingsServer<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"set\0\0\0\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<ISettingsServer<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<ISettingsServer<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"set\0\0\0\0\0") {
			let ret = Arc::new(ISettingsServer(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<ISettingsServer<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ISettingsServer(domain)),
			Err((sess, err)) => Err((ISettingsServer(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ISettingsServer<Session>> {
		Ok(ISettingsServer(self.0.duplicate()?))
	}
}

impl<T> Deref for ISettingsServer<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ISettingsServer<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ISettingsServer<T> {
	pub fn get_language_code(&self, ) -> Result<::ipcdefs::nn::settings::LanguageCode> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let res : Response<::ipcdefs::nn::settings::LanguageCode> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_available_language_codes(&self, unk1: &mut [::ipcdefs::nn::settings::LanguageCode]) -> Result<i32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			.descriptor(IPCBuffer::from_mut_slice(unk1, 0xa))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn make_language_code(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_available_language_code_count(&self, ) -> Result<i32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(3)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_region_code(&self, ) -> Result<i32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(4)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn get_available_language_codes2(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(5)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn get_available_language_code_count2(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(6)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn get_key_code_map(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(7)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn get_quest_flag2(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(8)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for ISettingsServer<T> {
	fn from(obj: T) -> ISettingsServer<T> {
		ISettingsServer(obj)
	}
}
