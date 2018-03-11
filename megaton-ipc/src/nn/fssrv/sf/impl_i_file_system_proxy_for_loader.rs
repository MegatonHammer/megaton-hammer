
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IFileSystemProxyForLoader(Session);

impl IFileSystemProxyForLoader {
	pub fn new() -> Result<IFileSystemProxyForLoader> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"fsp-ldr\0").map(|s| unsafe { IFileSystemProxyForLoader::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IFileSystemProxyForLoader {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IFileSystemProxyForLoader {
	// fn mount_code(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn is_code_mounted(&self, tid: ::nn::ApplicationId) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(tid)
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IFileSystemProxyForLoader {
	unsafe fn from_kobject(obj: KObject) -> IFileSystemProxyForLoader {
		IFileSystemProxyForLoader(Session::from_kobject(obj))
	}
}
