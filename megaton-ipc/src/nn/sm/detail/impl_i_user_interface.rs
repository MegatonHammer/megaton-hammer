
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IUserInterface<T>(T);

impl IUserInterface<Session> {
	pub fn raw_new() -> Result<IUserInterface<Session>> {
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let (r, session) = unsafe { svc::connect_to_named_port("sm:".as_ptr()) };
		if r != 0 {
			return Err(Error(r))
		} else {
			let ret = Session::from(unsafe { KObject::new(session) }).into();
			return Ok(ret);
		}
	}

	pub fn new() -> Result<Arc<IUserInterface<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IUserInterface<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"sm:\0\0\0\0\0") {
			let ret = Arc::new(IUserInterface(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IUserInterface<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IUserInterface(domain)),
			Err((sess, err)) => Err((IUserInterface(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IUserInterface<Session>> {
		Ok(IUserInterface(self.0.duplicate()?))
	}
}

impl<T> Deref for IUserInterface<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IUserInterface<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IUserInterface<T> {
	pub fn initialize(&self, reserved: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(reserved)
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_service(&self, name: ::ServiceName) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(name)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn register_service(&self, name: ::ServiceName, unk1: u8, max_handles: u32) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			name: ::ServiceName,
			unk1: u8,
			max_handles: u32,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(InRaw {
				name,
				unk1,
				max_handles,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn unregister_service(&self, name: ::ServiceName) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(3)
			.args(name)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IUserInterface<T> {
	fn from(obj: T) -> IUserInterface<T> {
		IUserInterface(obj)
	}
}
