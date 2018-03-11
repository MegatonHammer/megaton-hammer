
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IWriter(Session);

impl IWriter {
	pub fn new() -> Result<IWriter> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"arp:w\0\0\0").map(|s| unsafe { IWriter::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IWriter {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IWriter {
	pub fn get_i_registrar(&self, ) -> Result<Session> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn submit_writer(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IWriter {
	unsafe fn from_kobject(obj: KObject) -> IWriter {
		IWriter(Session::from_kobject(obj))
	}
}
