
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IUserInterface(Session);

impl IUserInterface {
	pub fn Initialize(&self, reserved: u64) -> Result<()> {
		let req = Request::new(0)
			.args(reserved)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn GetService(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn RegisterService(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn UnregisterService(&self, name: ::ServiceName) -> Result<()> {
		let req = Request::new(3)
			.args(name)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
}

impl FromKObject for IUserInterface {
	unsafe fn from_kobject(obj: KObject) -> IUserInterface {
		IUserInterface(Session::from_kobject(obj))
	}
}
