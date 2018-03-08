
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct ILogService(Session);

impl ILogService {
	pub fn get_service() -> Result<ILogService> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"lm\0\0\0\0\0\0").map(|s| unsafe { ILogService::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl ILogService {
	pub fn Initialize(&self, unk0: u64) -> Result<::nn::lm::ILogger> {
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
