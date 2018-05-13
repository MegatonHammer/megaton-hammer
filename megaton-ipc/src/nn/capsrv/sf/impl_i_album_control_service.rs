
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IAlbumControlService<T>(T);

impl IAlbumControlService<Session> {
	pub fn raw_new() -> Result<IAlbumControlService<Session>> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"caps:c\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IAlbumControlService<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IAlbumControlService<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"caps:c\0\0") {
			let ret = Arc::new(IAlbumControlService(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IAlbumControlService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IAlbumControlService(domain)),
			Err((sess, err)) => Err((IAlbumControlService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IAlbumControlService<Session>> {
		Ok(IAlbumControlService(self.0.duplicate()?))
	}
}

impl<T> Deref for IAlbumControlService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IAlbumControlService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IAlbumControlService<T> {
	pub fn unknown2001(&self, unk0: u8) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2001)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown2002(&self, unk0: u8) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2002)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown2011(&self, unk0: u64, unk1: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u64,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2011)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown2012(&self, unk0: u64, unk1: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u64,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2012)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown2013(&self, unk0: u64) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2013)
			.args(unk0)
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn unknown2014(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2014)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown2101(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown2102(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown2201(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown2301(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IAlbumControlService<T> {
	fn from(obj: T) -> IAlbumControlService<T> {
		IAlbumControlService(obj)
	}
}
