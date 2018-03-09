
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IApplicationProxyService(Session);

impl IApplicationProxyService {
	pub fn get_service() -> Result<IApplicationProxyService> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"appletOE").map(|s| unsafe { IApplicationProxyService::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IApplicationProxyService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IApplicationProxyService {
	pub fn OpenApplicationProxy(&self, unk0: u64, unk2: &KObject) -> Result<::nn::am::service::IApplicationProxy> {
		let req = Request::new(0)
			.args(unk0)
			.send_pid()
			.copy_handle(unk2)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IApplicationProxyService {
	unsafe fn from_kobject(obj: KObject) -> IApplicationProxyService {
		IApplicationProxyService(Session::from_kobject(obj))
	}
}
