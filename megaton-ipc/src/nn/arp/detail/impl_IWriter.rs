
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IWriter(Session);

impl IWriter {
	pub fn get_service() -> Result<IWriter> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"arp:w\0\0\0").map(|s| unsafe { IWriter::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl IWriter {
	pub fn GetIRegistrar(&self, ) -> Result<Session> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn SubmitWriter(&self, unk0: u64) -> Result<()> {
		let req = Request::new(1)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IWriter {
	unsafe fn from_kobject(obj: KObject) -> IWriter {
		IWriter(Session::from_kobject(obj))
	}
}
