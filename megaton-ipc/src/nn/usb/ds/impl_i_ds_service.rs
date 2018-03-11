
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IDsService(Session);

impl IDsService {
	pub fn new() -> Result<IDsService> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"usb:ds\0\0").map(|s| unsafe { IDsService::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IDsService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IDsService {
	pub fn bind_device(&self, complex_id: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(complex_id)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn bind_client_process(&self, unk0: &KObject) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			.copy_handle(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_ds_interface(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_state_change_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn get_state(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn set_vid_pid_bcd(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IDsService {
	unsafe fn from_kobject(obj: KObject) -> IDsService {
		IDsService(Session::from_kobject(obj))
	}
}
