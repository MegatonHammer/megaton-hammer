
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IAccountServiceForAdministrator<T>(T);

impl IAccountServiceForAdministrator<Session> {
	pub fn new() -> Result<Arc<IAccountServiceForAdministrator<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IAccountServiceForAdministrator<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"acc:su\0\0") {
			let ret = Arc::new(IAccountServiceForAdministrator(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"acc:su\0\0").map(|s: KObject| Arc::new(Session::from(s).into()));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
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
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_user_existence(&self, unk0: ::nn::account::Uid) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(unk0)
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn list_all_users(&self, unk0: &mut [::nn::account::Uid]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			.descriptor(IPCBuffer::from_mut_slice(unk0, 0xa))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn list_open_users(&self, unk0: &mut [::nn::account::Uid]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			.descriptor(IPCBuffer::from_mut_slice(unk0, 0xa))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_last_opened_user(&self, ) -> Result<::nn::account::Uid> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(())
			;
		let res : Response<::nn::account::Uid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_profile(&self, unk0: ::nn::account::Uid) -> Result<::nn::account::profile::IProfile<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(5)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_profile_digest(&self, unk0: ::nn::account::Uid) -> Result<::nn::account::ProfileDigest> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(6)
			.args(unk0)
			;
		let res : Response<::nn::account::ProfileDigest> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn is_user_registration_request_permitted(&self, unk0: u64) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(50)
			.args(unk0)
			.send_pid()
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn try_select_user_without_interaction(&self, unk0: bool) -> Result<::nn::account::Uid> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(51)
			.args(unk0)
			;
		let res : Response<::nn::account::Uid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_user_registration_notifier(&self, ) -> Result<::nn::account::detail::INotifier<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(100)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_user_state_change_notifier(&self, ) -> Result<::nn::account::detail::INotifier<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(101)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_baas_account_manager_for_system_service(&self, unk0: ::nn::account::Uid) -> Result<::nn::account::baas::IManagerForSystemService<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(102)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_baas_user_availability_change_notifier(&self, ) -> Result<::nn::account::detail::INotifier<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(103)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_profile_update_notifier(&self, ) -> Result<::nn::account::detail::INotifier<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(104)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	// fn store_save_data_thumbnail(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn clear_save_data_thumbnail(&self, unk0: ::nn::account::Uid, unk1: ::nn::ApplicationId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::account::Uid,
			unk1: ::nn::ApplicationId,
		}
		let req = Request::new(111)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn load_save_data_thumbnail(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_user_last_opened_application(&self, unk0: ::nn::account::Uid) -> Result<(u32, ::nn::ApplicationId)> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(190)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: u32,
			unk2: ::nn::ApplicationId,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().unk2.clone()))
	}

	pub fn begin_user_registration(&self, ) -> Result<::nn::account::Uid> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(200)
			.args(())
			;
		let res : Response<::nn::account::Uid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn complete_user_registration(&self, unk0: ::nn::account::Uid) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(201)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn cancel_user_registration(&self, unk0: ::nn::account::Uid) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(202)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn delete_user(&self, unk0: ::nn::account::Uid) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(203)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_user_position(&self, unk0: i32, unk1: ::nn::account::Uid) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::nn::account::Uid,
		}
		let req = Request::new(204)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_profile_editor(&self, unk0: ::nn::account::Uid) -> Result<::nn::account::profile::IProfileEditor<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(205)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn complete_user_registration_forcibly(&self, unk0: ::nn::account::Uid) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(206)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn create_floating_registration_request(&self, unk0: u32, unk1: &KObject) -> Result<::nn::account::baas::IFloatingRegistrationRequest<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(210)
			.args(unk0)
			.copy_handle(unk1)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn authenticate_service_async(&self, ) -> Result<::nn::account::detail::IAsyncContext<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(230)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_baas_account_administrator(&self, unk0: ::nn::account::Uid) -> Result<::nn::account::baas::IAdministrator<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(250)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn proxy_procedure_for_guest_login_with_nintendo_account(&self, unk0: ::nn::account::detail::Uuid) -> Result<::nn::account::nas::IOAuthProcedureForExternalNsa<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(290)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn proxy_procedure_for_floating_registration_with_nintendo_account(&self, unk0: ::nn::account::detail::Uuid) -> Result<::nn::account::nas::IOAuthProcedureForExternalNsa<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(291)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn suspend_background_daemon(&self, ) -> Result<::nn::account::detail::ISessionObject<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(299)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn debug_invalidate_token_cache_for_user(&self, unk0: ::nn::account::Uid) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(997)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn debug_set_user_state_close(&self, unk0: ::nn::account::Uid) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(998)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn debug_set_user_state_open(&self, unk0: ::nn::account::Uid) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(999)
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
