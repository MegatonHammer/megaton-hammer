
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IETicketService<T>(T);

impl IETicketService<Session> {
	pub fn new() -> Result<Arc<IETicketService<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IETicketService<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"es\0\0\0\0\0\0") {
			let ret = Arc::new(IETicketService(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"es\0\0\0\0\0\0").map(|s: KObject| Arc::new(Session::from(s).into()));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}

	pub fn to_domain(self) -> ::core::result::Result<IETicketService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IETicketService(domain)),
			Err((sess, err)) => Err((IETicketService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IETicketService<Session>> {
		Ok(IETicketService(self.0.duplicate()?))
	}
}

impl<T> Deref for IETicketService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IETicketService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IETicketService<T> {
	// fn unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown3(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown4(&self, unk0: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown5(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(5)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown6(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(6)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown7(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown8(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown9(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(9)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn unknown10(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn unknown11(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown12(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown13(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown14(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown15(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown16(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown17(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown18(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown19(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown20(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown21(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IETicketService<T> {
	fn from(obj: T) -> IETicketService<T> {
		IETicketService(obj)
	}
}
