
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IResolver(Session);

impl IResolver {
	pub fn new() -> Result<Arc<IResolver>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IResolver>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"sfdnsres") {
			let ret = Arc::new(IResolver(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"sfdnsres").map(|s| Arc::new(unsafe { IResolver::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IResolver {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IResolver {
	// fn unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_host_by_name(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_host_by_addr(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_host_string_error(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_gai_string_error(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_addr_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_name_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn request_cancel_handle(&self, unk0: u64) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(8)
			.args(unk0)
			.send_pid()
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn cancel_socket_call(&self, unk0: u32, unk1: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u64,
		}
		let req = Request::new(9)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown10(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn clear_dns_ip_server_address_array(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(11)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IResolver {
	unsafe fn from_kobject(obj: KObject) -> IResolver {
		IResolver(Session::from_kobject(obj))
	}
}
