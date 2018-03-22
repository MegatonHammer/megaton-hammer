
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IGeneralInterface(Session);

impl IGeneralInterface {
	pub fn new() -> Result<Arc<IGeneralInterface>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IGeneralInterface>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"spl:\0\0\0\0") {
			let ret = Arc::new(IGeneralInterface(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"spl:\0\0\0\0").map(|s| Arc::new(unsafe { IGeneralInterface::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IGeneralInterface {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IGeneralInterface {
	pub fn get_config(&self, unk0: u32) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(unk0)
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn user_exp_mod(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn generate_aes_kek(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn load_aes_key(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn generate_aes_key(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_config(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_random_bytes(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn load_secure_exp_mod_key(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn secure_exp_mod(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn is_development(&self, ) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(11)
			.args(())
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn generate_specific_aes_key(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn decrypt_privk(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn decrypt_aes_key(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn decrypt_aes_ctr(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn compute_cmac(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn load_rsa_oaep_key(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unwrap_rsa_oaep_wrapped_title_key(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn load_title_key(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unwrap_aes_wrapped_title_key(&self, UNKNOWN) -> Result<UNKNOWN>;
	#[cfg(feature = "switch-2.0.0")]
	pub fn lock_aes_engine(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(21)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn unlock_aes_engine(&self, unk0: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(22)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn get_spl_wait_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(23)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn set_shared_data(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_shared_data(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IGeneralInterface {
	unsafe fn from_kobject(obj: KObject) -> IGeneralInterface {
		IGeneralInterface(Session::from_kobject(obj))
	}
}
