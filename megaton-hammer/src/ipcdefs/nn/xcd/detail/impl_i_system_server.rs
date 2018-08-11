
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::sync::Arc;

#[derive(Debug)]
pub struct ISystemServer<T>(T);

impl ISystemServer<Session> {
	pub fn raw_new() -> Result<ISystemServer<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"xcd:sys\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<ISystemServer<Session>>> {
		use alloc::sync::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<ISystemServer<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"xcd:sys\0") {
			let ret = Arc::new(ISystemServer(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<ISystemServer<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ISystemServer(domain)),
			Err((sess, err)) => Err((ISystemServer(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ISystemServer<Session>> {
		Ok(ISystemServer(self.0.duplicate()?))
	}
}

impl<T> Deref for ISystemServer<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ISystemServer<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ISystemServer<T> {
	pub fn get_data_format(&self, unk0: u64) -> Result<u8> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(unk0)
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn set_data_format(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_mcu_state(&self, unk0: u64) -> Result<u8> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(unk0)
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn set_mcu_state(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_mcu_version_for_nfc(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn check_nfc_device_power(&self, unk0: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(5)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn set_nfc_event(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_nfc_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn start_nfc_discovery(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn stop_nfc_discovery(&self, unk0: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(13)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn start_ntag_read(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn start_ntag_write(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn send_nfc_raw_data(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn register_mifare_key(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn clear_mifare_key(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn start_mifare_read(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn start_mifare_write(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_awake_trigger_reason_for_left_rail(&self, ) -> Result<u64> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(101)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_awake_trigger_reason_for_right_rail(&self, ) -> Result<u64> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(102)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl<T: Object> From<T> for ISystemServer<T> {
	fn from(obj: T) -> ISystemServer<T> {
		ISystemServer(obj)
	}
}
