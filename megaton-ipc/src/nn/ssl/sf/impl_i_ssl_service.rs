
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct ISslService(Session);

impl ISslService {
	pub fn new() -> Result<Arc<ISslService>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<ISslService>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"ssl\0\0\0\0\0") {
			let ret = Arc::new(ISslService(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"ssl\0\0\0\0\0").map(|s| Arc::new(unsafe { ISslService::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for ISslService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ISslService {
	pub fn create_context(&self, unk0: ::nn::ssl::sf::SslVersion, unk1: u64) -> Result<::nn::ssl::sf::ISslContext> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::ssl::sf::SslVersion,
			unk1: u64,
		}
		let req = Request::new(0)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn get_context_count(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_certificates(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_certificate_buf_size(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn debug_ioctl(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn set_interface_version(&self, unk0: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(5)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for ISslService {
	unsafe fn from_kobject(obj: KObject) -> ISslService {
		ISslService(Session::from_kobject(obj))
	}
}
