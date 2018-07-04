
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IGeneralInterface<T>(T);

impl IGeneralInterface<Session> {
	pub fn raw_new() -> Result<IGeneralInterface<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"spl:\0\0\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IGeneralInterface<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IGeneralInterface<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"spl:\0\0\0\0") {
			let ret = Arc::new(IGeneralInterface(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IGeneralInterface<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IGeneralInterface(domain)),
			Err((sess, err)) => Err((IGeneralInterface(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IGeneralInterface<Session>> {
		Ok(IGeneralInterface(self.0.duplicate()?))
	}
}

impl<T> Deref for IGeneralInterface<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IGeneralInterface<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IGeneralInterface<T> {
	pub fn get_config(&self, unk0: u32) -> Result<u64> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
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
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(11)
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
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(21)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn unlock_aes_engine(&self, unk0: u32) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(22)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn get_spl_wait_event(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(23)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn set_shared_data(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_shared_data(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IGeneralInterface<T> {
	fn from(obj: T) -> IGeneralInterface<T> {
		IGeneralInterface(obj)
	}
}
