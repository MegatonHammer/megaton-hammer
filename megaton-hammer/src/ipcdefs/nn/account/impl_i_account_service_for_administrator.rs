
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::sync::Arc;

#[derive(Debug)]
pub struct IAccountServiceForAdministrator<T>(T);

impl IAccountServiceForAdministrator<Session> {
	pub fn raw_new() -> Result<IAccountServiceForAdministrator<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"acc:su\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IAccountServiceForAdministrator<Session>>> {
		use alloc::sync::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IAccountServiceForAdministrator<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"acc:su\0\0") {
			let ret = Arc::new(IAccountServiceForAdministrator(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IAccountServiceForAdministrator<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IAccountServiceForAdministrator(domain)),
			Err((sess, err)) => Err((IAccountServiceForAdministrator(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IAccountServiceForAdministrator<Session>> {
		Ok(IAccountServiceForAdministrator(self.0.duplicate()?))
	}
}

impl<T> Deref for IAccountServiceForAdministrator<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IAccountServiceForAdministrator<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IAccountServiceForAdministrator<T> {
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
	pub fn get_user_registration_notifier(&self, ) -> Result<::ipcdefs::nn::account::detail::INotifier<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(100)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_user_state_change_notifier(&self, ) -> Result<::ipcdefs::nn::account::detail::INotifier<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(101)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_baas_account_manager_for_system_service(&self, unk0: ::ipcdefs::nn::account::Uid) -> Result<::ipcdefs::nn::account::baas::IManagerForSystemService<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(102)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_baas_user_availability_change_notifier(&self, ) -> Result<::ipcdefs::nn::account::detail::INotifier<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(103)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_profile_update_notifier(&self, ) -> Result<::ipcdefs::nn::account::detail::INotifier<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(104)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	// fn check_network_service_availability_async(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn store_save_data_thumbnail(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn clear_save_data_thumbnail(&self, unk0: ::ipcdefs::nn::account::Uid, unk1: ::ipcdefs::nn::ApplicationId) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::account::Uid,
			unk1: ::ipcdefs::nn::ApplicationId,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(111)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn load_save_data_thumbnail(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_save_data_thumbnail_existence(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_user_last_opened_application(&self, unk0: ::ipcdefs::nn::account::Uid) -> Result<(u32, ::ipcdefs::nn::ApplicationId)> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(190)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: u32,
			unk2: ::ipcdefs::nn::ApplicationId,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().unk2.clone()))
	}

	// fn activate_open_context_holder(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn begin_user_registration(&self, ) -> Result<::ipcdefs::nn::account::Uid> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(200)
			.args(())
			;
		let res : Response<::ipcdefs::nn::account::Uid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn complete_user_registration(&self, unk0: ::ipcdefs::nn::account::Uid) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(201)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn cancel_user_registration(&self, unk0: ::ipcdefs::nn::account::Uid) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(202)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn delete_user(&self, unk0: ::ipcdefs::nn::account::Uid) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(203)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_user_position(&self, unk0: i32, unk1: ::ipcdefs::nn::account::Uid) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::ipcdefs::nn::account::Uid,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(204)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_profile_editor(&self, unk0: ::ipcdefs::nn::account::Uid) -> Result<::ipcdefs::nn::account::profile::IProfileEditor<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(205)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn complete_user_registration_forcibly(&self, unk0: ::ipcdefs::nn::account::Uid) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(206)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn create_floating_registration_request(&self, unk0: u32, unk1: &KObject) -> Result<::ipcdefs::nn::account::baas::IFloatingRegistrationRequest<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 1], [_; 0]> = Request::new(210)
			.args(unk0)
			.copy_handle(unk1)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn authenticate_service_async(&self, ) -> Result<::ipcdefs::nn::account::detail::IAsyncContext<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(230)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_baas_account_administrator(&self, unk0: ::ipcdefs::nn::account::Uid) -> Result<::ipcdefs::nn::account::baas::IAdministrator<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(250)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn proxy_procedure_for_guest_login_with_nintendo_account(&self, unk0: ::ipcdefs::nn::account::detail::Uuid) -> Result<::ipcdefs::nn::account::nas::IOAuthProcedureForExternalNsa<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(290)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn proxy_procedure_for_floating_registration_with_nintendo_account(&self, unk0: ::ipcdefs::nn::account::detail::Uuid) -> Result<::ipcdefs::nn::account::nas::IOAuthProcedureForExternalNsa<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(291)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn suspend_background_daemon(&self, ) -> Result<::ipcdefs::nn::account::detail::ISessionObject<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(299)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn debug_invalidate_token_cache_for_user(&self, unk0: ::ipcdefs::nn::account::Uid) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(997)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn debug_set_user_state_close(&self, unk0: ::ipcdefs::nn::account::Uid) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(998)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn debug_set_user_state_open(&self, unk0: ::ipcdefs::nn::account::Uid) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(999)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IAccountServiceForAdministrator<T> {
	fn from(obj: T) -> IAccountServiceForAdministrator<T> {
		IAccountServiceForAdministrator(obj)
	}
}
