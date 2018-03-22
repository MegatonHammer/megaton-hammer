
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IAccountServiceForApplication(Session);

impl IAccountServiceForApplication {
	pub fn new() -> Result<Arc<IAccountServiceForApplication>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IAccountServiceForApplication>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"acc:u0\0\0") {
			let ret = Arc::new(IAccountServiceForApplication(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"acc:u0\0\0").map(|s| Arc::new(unsafe { IAccountServiceForApplication::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IAccountServiceForApplication {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IAccountServiceForApplication {
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

	pub fn initialize_application_info(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(100)
			.args(unk0)
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_baas_account_manager_for_application(&self, unk0: ::nn::account::Uid) -> Result<::nn::account::baas::IManagerForApplication> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(101)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn authenticate_application_async(&self, ) -> Result<::nn::account::detail::IAsyncContext> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(102)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	// fn store_save_data_thumbnail(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn clear_save_data_thumbnail(&self, unk0: ::nn::account::Uid) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(111)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn create_guest_login_request(&self, unk0: u32, unk1: &KObject) -> Result<::nn::account::baas::IGuestLoginRequest> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(120)
			.args(unk0)
			.copy_handle(unk1)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IAccountServiceForApplication {
	unsafe fn from_kobject(obj: KObject) -> IAccountServiceForApplication {
		IAccountServiceForApplication(Session::from_kobject(obj))
	}
}
