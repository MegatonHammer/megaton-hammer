
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IManager(Session);

impl IManager {
	pub fn new() -> Result<Arc<IManager>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IManager>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"uart\0\0\0\0") {
			let ret = Arc::new(IManager(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"uart\0\0\0\0").map(|s| Arc::new(unsafe { IManager::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IManager {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IManager {
	pub fn does_uart_exist(&self, unk0: u32) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(unk0)
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn does_uart_exist_for_test(&self, unk0: u32) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(unk0)
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn set_uart_baudrate(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_uart_baudrate_for_test(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn is_something_uart_valid(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn is_something_uart_valid_for_test(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_session(&self, ) -> Result<Session> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(6)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	// fn is_something_uart_valid2(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn is_something_uart_valid2_for_test(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IManager {
	unsafe fn from_kobject(obj: KObject) -> IManager {
		IManager(Session::from_kobject(obj))
	}
}
