
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IRegistrar(Session);

impl AsRef<Session> for IRegistrar {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IRegistrar {
	pub fn bind_registrar(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn write_header(&self, unk0: u128) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn write_data(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IRegistrar {
	unsafe fn from_kobject(obj: KObject) -> IRegistrar {
		IRegistrar(Session::from_kobject(obj))
	}
}
