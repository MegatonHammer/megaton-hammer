
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IQueryService(Session);

impl IQueryService {
	pub fn new() -> Result<Arc<IQueryService>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IQueryService>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"pdm:qry\0") {
			let ret = Arc::new(IQueryService(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"pdm:qry\0").map(|s| Arc::new(unsafe { IQueryService::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IQueryService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IQueryService {
	// fn unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown3(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown4(&self, unk0: u64) -> Result<(u64, u64, u64, u64, u64)> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: u64,
			unk2: u64,
			unk3: u64,
			unk4: u64,
			unk5: u64,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().unk2.clone(),res.get_raw().unk3.clone(),res.get_raw().unk4.clone(),res.get_raw().unk5.clone()))
	}

	pub fn unknown5(&self, unk0: u64, unk1: u64, unk2: u64) -> Result<(u64, u64, u64, u64, u64)> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u64,
			unk2: u64,
		}
		let req = Request::new(5)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk3: u64,
			unk4: u64,
			unk5: u64,
			unk6: u64,
			unk7: u64,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk3.clone(),res.get_raw().unk4.clone(),res.get_raw().unk5.clone(),res.get_raw().unk6.clone(),res.get_raw().unk7.clone()))
	}

	pub fn unknown6(&self, unk0: u64, unk1: u64) -> Result<(u64, u64, u64, u64, u64)> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u64,
		}
		let req = Request::new(6)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk2: u64,
			unk3: u64,
			unk4: u64,
			unk5: u64,
			unk6: u64,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk2.clone(),res.get_raw().unk3.clone(),res.get_raw().unk4.clone(),res.get_raw().unk5.clone(),res.get_raw().unk6.clone()))
	}

	// fn unknown7(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown8(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown9(&self, ) -> Result<(u32, u32, u32)> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(9)
			.args(())
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk0: u32,
			unk1: u32,
			unk2: u32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk0.clone(),res.get_raw().unk1.clone(),res.get_raw().unk2.clone()))
	}

	// fn unknown10(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IQueryService {
	unsafe fn from_kobject(obj: KObject) -> IQueryService {
		IQueryService(Session::from_kobject(obj))
	}
}
