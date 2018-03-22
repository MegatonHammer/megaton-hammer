
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IBoardPowerControlManager(Session);

impl IBoardPowerControlManager {
	pub fn new() -> Result<Arc<IBoardPowerControlManager>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IBoardPowerControlManager>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"bpc\0\0\0\0\0") {
			let ret = Arc::new(IBoardPowerControlManager(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"bpc\0\0\0\0\0").map(|s| Arc::new(unsafe { IBoardPowerControlManager::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IBoardPowerControlManager {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IBoardPowerControlManager {
	pub fn shutdown_system(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn reboot_system(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_wakeup_reason(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_shutdown_reason(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_ac_ok(&self, ) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(())
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_board_power_control_event(&self, unk0: u32) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(5)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn get_sleep_button_state(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(6)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_power_event(&self, unk0: u32) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(7)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn unknown8(&self, unk0: u64) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(8)
			.args(unk0)
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn unknown9(&self, unk0: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(9)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown10(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IBoardPowerControlManager {
	unsafe fn from_kobject(obj: KObject) -> IBoardPowerControlManager {
		IBoardPowerControlManager(Session::from_kobject(obj))
	}
}
