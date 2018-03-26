
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IPdCradleSession<T>(T);

impl IPdCradleSession<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IPdCradleSession<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IPdCradleSession(domain)),
			Err((sess, err)) => Err((IPdCradleSession(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IPdCradleSession<Session>> {
		Ok(IPdCradleSession(self.0.duplicate()?))
	}
}

impl<T> Deref for IPdCradleSession<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IPdCradleSession<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IPdCradleSession<T> {
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

impl<T: Object> From<T> for IPdCradleSession<T> {
	fn from(obj: T) -> IPdCradleSession<T> {
		IPdCradleSession(obj)
	}
}
