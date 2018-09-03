
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::sync::Arc;

#[derive(Debug)]
pub struct IAccountServiceForApplication<T>(T);

impl IAccountServiceForApplication<Session> {
	pub fn raw_new() -> Result<IAccountServiceForApplication<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"acc:u0\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IAccountServiceForApplication<Session>>> {
		use alloc::sync::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IAccountServiceForApplication<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"acc:u0\0\0") {
			let ret = Arc::new(IAccountServiceForApplication(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IAccountServiceForApplication<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IAccountServiceForApplication(domain)),
			Err((sess, err)) => Err((IAccountServiceForApplication(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IAccountServiceForApplication<Session>> {
		Ok(IAccountServiceForApplication(self.0.duplicate()?))
	}
}

impl<T> Deref for IAccountServiceForApplication<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IAccountServiceForApplication<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IAccountServiceForApplication<T> {
	pub fn get_user_count(&self, ) -> Result<i32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_user_existence(&self, unk0: ::ipcdefs::nn::account::Uid) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(unk0)
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn list_all_users(&self, unk0: &mut [::ipcdefs::nn::account::Uid]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			.descriptor(IPCBuffer::from_mut_slice(unk0, 0xa))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn list_open_users(&self, unk0: &mut [::ipcdefs::nn::account::Uid]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(3)
			.args(())
			.descriptor(IPCBuffer::from_mut_slice(unk0, 0xa))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_last_opened_user(&self, ) -> Result<::ipcdefs::nn::account::Uid> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(4)
			.args(())
			;
		let res : Response<::ipcdefs::nn::account::Uid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_profile(&self, unk0: ::ipcdefs::nn::account::Uid) -> Result<::ipcdefs::nn::account::profile::IProfile<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(5)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_profile_digest(&self, unk0: ::ipcdefs::nn::account::Uid) -> Result<::ipcdefs::nn::account::ProfileDigest> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(6)
			.args(unk0)
			;
		let res : Response<::ipcdefs::nn::account::ProfileDigest> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn is_user_registration_request_permitted(&self, unk0: u64) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(50)
			.args(unk0)
			.send_pid()
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn try_select_user_without_interaction(&self, unk0: bool) -> Result<::ipcdefs::nn::account::Uid> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(51)
			.args(unk0)
			;
		let res : Response<::ipcdefs::nn::account::Uid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn list_open_context_stored_users(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn initialize_application_info(&self, unk0: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(100)
			.args(unk0)
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_baas_account_manager_for_application(&self, unk0: ::ipcdefs::nn::account::Uid) -> Result<::ipcdefs::nn::account::baas::IManagerForApplication<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(101)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn authenticate_application_async(&self, ) -> Result<::ipcdefs::nn::account::detail::IAsyncContext<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(102)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	// fn check_network_service_availability_async(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn store_save_data_thumbnail(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn clear_save_data_thumbnail(&self, unk0: ::ipcdefs::nn::account::Uid) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(111)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn create_guest_login_request(&self, unk0: u32, unk1: &KObject) -> Result<::ipcdefs::nn::account::baas::IGuestLoginRequest<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 1], [_; 0]> = Request::new(120)
			.args(unk0)
			.copy_handle(unk1)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	// fn load_open_context(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IAccountServiceForApplication<T> {
	fn from(obj: T) -> IAccountServiceForApplication<T> {
		IAccountServiceForApplication(obj)
	}
}
