
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IFileSystemProxyForLoader(Session);

impl IFileSystemProxyForLoader {
	pub fn MountCode(&self, TID: ::nn::ApplicationId, contentPath: &i8) -> Result<(::nn::fssrv::sf::IFileSystem)> {
		let req = Request::new(0)
			.args(TID)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn IsCodeMounted(&self, TID: ::nn::ApplicationId) -> Result<(u8)> {
		let req = Request::new(1)
			.args(TID)
			;
		let mut res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IFileSystemProxyForLoader {
	unsafe fn from_kobject(obj: KObject) -> IFileSystemProxyForLoader {
		IFileSystemProxyForLoader(Session::from_kobject(obj))
	}
}
