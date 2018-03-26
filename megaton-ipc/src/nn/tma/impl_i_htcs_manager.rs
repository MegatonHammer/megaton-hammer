
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IHtcsManager<T>(T);

impl IHtcsManager<Session> {
	pub fn new() -> Result<Arc<IHtcsManager<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IHtcsManager<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"htcs\0\0\0\0") {
			let ret = Arc::new(IHtcsManager(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"htcs\0\0\0\0").map(|s: KObject| Arc::new(Session::from(s).into()));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}

	pub fn to_domain(self) -> ::core::result::Result<IHtcsManager<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IHtcsManager(domain)),
			Err((sess, err)) => Err((IHtcsManager(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IHtcsManager<Session>> {
		Ok(IHtcsManager(self.0.duplicate()?))
	}
}

impl<T> Deref for IHtcsManager<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IHtcsManager<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IHtcsManager<T> {
	pub fn unknown0(&self, ) -> Result<(u32, u32)> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk0: u32,
			unk1: u32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk0.clone(),res.get_raw().unk1.clone()))
	}

	pub fn unknown1(&self, unk0: u32) -> Result<(u32, u32)> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: u32,
			unk2: u32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().unk2.clone()))
	}

	// fn unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown3(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown4(&self, unk0: u32, unk1: u32) -> Result<(u32, u32)> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u32,
		}
		let req = Request::new(4)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk2: u32,
			unk3: u32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk2.clone(),res.get_raw().unk3.clone()))
	}

	// fn unknown5(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown6(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown7(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown8(&self, unk0: u32, unk1: u32) -> Result<(u32, u32)> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u32,
		}
		let req = Request::new(8)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk2: u32,
			unk3: u32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk2.clone(),res.get_raw().unk3.clone()))
	}

	pub fn unknown9(&self, unk0: u32, unk1: u32, unk2: u32) -> Result<(u32, u32)> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u32,
			unk2: u32,
		}
		let req = Request::new(9)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk3: u32,
			unk4: u32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk3.clone(),res.get_raw().unk4.clone()))
	}

	// fn get_peer_name_any(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_default_host_name(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown12(&self, ) -> Result<(u32, T)> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(12)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok((*res.get_raw(),T::from_res(&mut res).into()))
	}

	pub fn create_socket(&self, unk0: u8) -> Result<(u32, T)> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(13)
			.args(unk0)
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok((*res.get_raw(),T::from_res(&mut res).into()))
	}

	pub fn register_process_id(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(100)
			.args(unk0)
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn monitor_manager(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(101)
			.args(unk0)
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IHtcsManager<T> {
	fn from(obj: T) -> IHtcsManager<T> {
		IHtcsManager(obj)
	}
}
