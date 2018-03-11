
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hammer::ipc::IPCBuffer;

#[derive(Debug)]
pub struct ILogger(Session);

impl AsRef<Session> for ILogger {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ILogger {
	// fn Unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown1(&self, unk0: u32) -> Result<()> {
		let req = Request::new(1)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for ILogger {
	unsafe fn from_kobject(obj: KObject) -> ILogger {
		ILogger(Session::from_kobject(obj))
	}
}
