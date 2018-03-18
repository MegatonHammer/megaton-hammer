
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IAccountServiceForSystemService(Session);

impl IAccountServiceForSystemService {
	pub fn new() -> Result<Arc<IAccountServiceForSystemService>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IAccountServiceForSystemService>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		let r = sm.get_service(*b"acc:u1\0\0").map(|s| Arc::new(unsafe { IAccountServiceForSystemService::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IAccountServiceForSystemService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IAccountServiceForSystemService {
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

	// fn list_all_users(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn list_open_users(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_last_opened_user(&self, ) -> Result<::nn::account::Uid> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(())
			;
		let res : Response<::nn::account::Uid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_profile(&self, unk0: ::nn::account::Uid) -> Result<::nn::account::profile::IProfile> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(5)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
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

	pub fn get_user_registration_notifier(&self, ) -> Result<::nn::account::detail::INotifier> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(100)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn get_user_state_change_notifier(&self, ) -> Result<::nn::account::detail::INotifier> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(101)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn get_baas_account_manager_for_system_service(&self, unk0: ::nn::account::Uid) -> Result<::nn::account::baas::IManagerForSystemService> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(102)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn get_baas_user_availability_change_notifier(&self, ) -> Result<::nn::account::detail::INotifier> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(103)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn get_profile_update_notifier(&self, ) -> Result<::nn::account::detail::INotifier> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(104)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
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

impl FromKObject for IAccountServiceForSystemService {
	unsafe fn from_kobject(obj: KObject) -> IAccountServiceForSystemService {
		IAccountServiceForSystemService(Session::from_kobject(obj))
	}
}
