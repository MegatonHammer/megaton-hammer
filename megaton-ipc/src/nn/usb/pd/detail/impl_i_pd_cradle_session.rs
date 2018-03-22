
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IPdCradleSession(Session);

impl AsRef<Session> for IPdCradleSession {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IPdCradleSession {
	pub fn vdm_user_write(&self, unk0: u32, unk1: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u32,
		}
		let req = Request::new(0)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn vdm_user_read(&self, unk0: u32) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(unk0)
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn vdm20_init(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_fw_type(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_fw_revision(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_manufacturer_id(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_device_id(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IPdCradleSession {
	unsafe fn from_kobject(obj: KObject) -> IPdCradleSession {
		IPdCradleSession(Session::from_kobject(obj))
	}
}
