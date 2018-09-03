
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::sync::Arc;

#[derive(Debug)]
pub struct ILocalManager<T>(T);

impl ILocalManager<Session> {
	pub fn raw_new() -> Result<ILocalManager<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"wlan:lcl")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<ILocalManager<Session>>> {
		use alloc::sync::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<ILocalManager<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"wlan:lcl") {
			let ret = Arc::new(ILocalManager(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<ILocalManager<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ILocalManager(domain)),
			Err((sess, err)) => Err((ILocalManager(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ILocalManager<Session>> {
		Ok(ILocalManager(self.0.duplicate()?))
	}
}

impl<T> Deref for ILocalManager<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ILocalManager<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ILocalManager<T> {
	pub fn unknown0(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown1(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown2(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown3(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(3)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown4(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(4)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown5(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(5)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_mac_address(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn create_bss(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn destroy_bss(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(8)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn start_scan(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn stop_scan(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(10)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn connect(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn cancel_connect(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(12)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn join(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn cancel_join(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(14)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn disconnect(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn set_beacon_lost_count(&self, unk0: u32) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(16)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown17(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown18(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown19(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_bss_indication_event(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(20)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn get_bss_indication_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_state(&self, ) -> Result<u32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(22)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_allowed_channels(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn add_ie(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn delete_ie(&self, unk0: u32) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(25)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown26(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown27(&self, unk0: u32) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(27)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn create_rx_entry(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn delete_rx_entry(&self, unk0: u32) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(29)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown30(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown31(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn add_matching_data_to_rx_entry(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn remove_matching_data_from_rx_entry(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_scan_result(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown35(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_action_frame_with_beacon(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn cancel_action_frame_with_beacon(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(37)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn create_rx_entry_for_action_frame(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn delete_rx_entry_for_action_frame(&self, unk0: u32) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(39)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown40(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown41(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn cancel_get_action_frame(&self, unk0: u32) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(42)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_rssi(&self, ) -> Result<u32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(43)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn unknown44(&self, unk0: u32) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(44)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown45(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown46(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown47(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown48(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for ILocalManager<T> {
	fn from(obj: T) -> ILocalManager<T> {
		ILocalManager(obj)
	}
}
