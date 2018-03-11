
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IInformationInterface(Session);

impl IInformationInterface {
	pub fn new() -> Result<IInformationInterface> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"pm:info\0").map(|s| unsafe { IInformationInterface::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IInformationInterface {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IInformationInterface {
	pub fn get_title_id(&self, unk0: u64) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(unk0)
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IInformationInterface {
	unsafe fn from_kobject(obj: KObject) -> IInformationInterface {
		IInformationInterface(Session::from_kobject(obj))
	}
}
