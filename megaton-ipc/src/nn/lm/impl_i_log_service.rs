
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct ILogService(Session);

impl ILogService {
	pub fn new() -> Result<ILogService> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"lm\0\0\0\0\0\0").map(|s| unsafe { ILogService::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for ILogService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ILogService {
	pub fn initialize(&self, unk0: u64) -> Result<::nn::lm::ILogger> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for ILogService {
	unsafe fn from_kobject(obj: KObject) -> ILogService {
		ILogService(Session::from_kobject(obj))
	}
}
