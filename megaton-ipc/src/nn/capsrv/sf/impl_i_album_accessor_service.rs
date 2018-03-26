
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IAlbumAccessorService<T>(T);

impl IAlbumAccessorService<Session> {
	pub fn new() -> Result<Arc<IAlbumAccessorService<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IAlbumAccessorService<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"caps:a\0\0") {
			let ret = Arc::new(IAlbumAccessorService(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"caps:a\0\0").map(|s: KObject| Arc::new(Session::from(s).into()));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}

	pub fn to_domain(self) -> ::core::result::Result<IAlbumAccessorService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IAlbumAccessorService(domain)),
			Err((sess, err)) => Err((IAlbumAccessorService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IAlbumAccessorService<Session>> {
		Ok(IAlbumAccessorService(self.0.duplicate()?))
	}
}

impl<T> Deref for IAlbumAccessorService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IAlbumAccessorService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IAlbumAccessorService<T> {
	// fn unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown3(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown4(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown5(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown6(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown7(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown8(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown9(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown10(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown11(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown12(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown13(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown14(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown301(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown401(&self, ) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(401)
			.args(())
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn unknown501(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown1001(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown1002(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown8001(&self, unk0: u8) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(8001)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown8002(&self, unk0: u8) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(8002)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown8011(&self, unk0: u8) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(8011)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown8012(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown8021(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown10011(&self, unk0: u8) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10011)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IAlbumAccessorService<T> {
	fn from(obj: T) -> IAlbumAccessorService<T> {
		IAlbumAccessorService(obj)
	}
}
