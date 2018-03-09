
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IReceiverService(Session);

impl IReceiverService {
	pub fn get_service() -> Result<IReceiverService> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"ovln:rcv").map(|s| unsafe { IReceiverService::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IReceiverService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IReceiverService {
	pub fn Unknown0(&self, ) -> Result<Session> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IReceiverService {
	unsafe fn from_kobject(obj: KObject) -> IReceiverService {
		IReceiverService(Session::from_kobject(obj))
	}
}
