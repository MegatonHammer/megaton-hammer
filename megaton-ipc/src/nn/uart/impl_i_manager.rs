
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IManager<T>(T);

impl IManager<Session> {
	pub fn raw_new() -> Result<IManager<Session>> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"uart\0\0\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IManager<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IManager<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"uart\0\0\0\0") {
			let ret = Arc::new(IManager(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IManager<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IManager(domain)),
			Err((sess, err)) => Err((IManager(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IManager<Session>> {
		Ok(IManager(self.0.duplicate()?))
	}
}

impl<T> Deref for IManager<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IManager<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IManager<T> {
	pub fn does_uart_exist(&self, unk0: u32) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(unk0)
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn does_uart_exist_for_test(&self, unk0: u32) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(unk0)
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn set_uart_baudrate(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_uart_baudrate_for_test(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn is_something_uart_valid(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn is_something_uart_valid_for_test(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_session(&self, ) -> Result<T> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(6)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	// fn is_something_uart_valid2(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn is_something_uart_valid2_for_test(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IManager<T> {
	fn from(obj: T) -> IManager<T> {
		IManager(obj)
	}
}
