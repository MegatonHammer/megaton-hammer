
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IServiceManager(Session);

impl AsRef<Session> for IServiceManager {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IServiceManager {
	pub fn unknown0(&self, unk0: u64) -> Result<Session> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IServiceManager {
	unsafe fn from_kobject(obj: KObject) -> IServiceManager {
		IServiceManager(Session::from_kobject(obj))
	}
}
