
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IInformationInterface(Session);

impl IInformationInterface {
	pub fn get_service() -> Result<IInformationInterface> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"pm:info\0").map(|s| unsafe { IInformationInterface::from_kobject(s) });
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
	pub fn GetTitleId(&self, unk0: u64) -> Result<u64> {
		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IInformationInterface {
	unsafe fn from_kobject(obj: KObject) -> IInformationInterface {
		IInformationInterface(Session::from_kobject(obj))
	}
}
