
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IRegistrar(Session);

impl IRegistrar {
	pub fn BindRegistrar(&self, unk0: u64) -> Result<()> {
		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn WriteHeader(&self, unk0: u128) -> Result<()> {
		let req = Request::new(1)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn WriteData(&self, ) -> Result<()> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
}

impl FromKObject for IRegistrar {
	unsafe fn from_kobject(obj: KObject) -> IRegistrar {
		IRegistrar(Session::from_kobject(obj))
	}
}
