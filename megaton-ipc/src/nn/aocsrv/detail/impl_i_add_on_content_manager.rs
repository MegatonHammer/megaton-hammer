
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IAddOnContentManager(Session);

impl IAddOnContentManager {
	pub fn new() -> Result<Arc<IAddOnContentManager>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IAddOnContentManager>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		let r = sm.get_service(*b"aoc:u\0\0\0").map(|s| Arc::new(unsafe { IAddOnContentManager::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IAddOnContentManager {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IAddOnContentManager {
	pub fn count_add_on_content_by_application_id(&self, unk0: ::nn::ncm::ApplicationId) -> Result<i32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(unk0)
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn list_add_on_content_by_application_id(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn count_add_on_content(&self, unk0: u64) -> Result<i32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(unk0)
			.send_pid()
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn list_add_on_content(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_add_on_content_base_id_by_application_id(&self, unk0: ::nn::ncm::ApplicationId) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(unk0)
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_add_on_content_base_id(&self, unk0: u64) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(5)
			.args(unk0)
			.send_pid()
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn prepare_add_on_content_by_application_id(&self, unk0: i32, unk1: ::nn::ncm::ApplicationId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::nn::ncm::ApplicationId,
		}
		let req = Request::new(6)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn prepare_add_on_content(&self, unk0: i32, unk1: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: u64,
		}
		let req = Request::new(7)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IAddOnContentManager {
	unsafe fn from_kobject(obj: KObject) -> IAddOnContentManager {
		IAddOnContentManager(Session::from_kobject(obj))
	}
}
